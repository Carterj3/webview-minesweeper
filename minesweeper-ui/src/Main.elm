port module Main exposing (main)

import Browser
import Html exposing (Html, Attribute, label, button, div, text, input, table, td, tr)
import Html.Attributes exposing (type_, value, placeholder)
import Html.Events exposing (onInput, custom)

import Json.Encode as JE
import Json.Decode as JD


main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }

-- MODEL

type GameState = InProgress
                | Won
                | Loss

type alias Tile =
    { num_bombs_around: Int
    , has_flag: Bool
    , is_bomb: Bool
    , is_shown: Bool
    , was_clicked: Bool
    }

type alias Model = 
    { debug: String
    , width: Int
    , height: Int
    , num_bombs: Int
    , state: GameState
    , field: List (List Tile)
    }

init : () -> (Model, Cmd Msg)
init _ =
  ( { debug= ""
    , width= 10
    , height= 10
    , num_bombs= 8
    , state= InProgress
    , field= [[ ]]
    }
  , Cmd.none
  )


-- UPDATE

sendUnflag: Int -> Int -> Cmd Msg
sendUnflag h w =
    let 
        json = JE.object    [ ("_type", JE.string "Unflag")
                            , ("x_position", JE.int w)
                            , ("y_position", JE.int h)
                            ]
        str = JE.encode 0 json
    in
        toBackEnd str

sendFlag: Int -> Int -> Cmd Msg
sendFlag h w=
    let 
        json = JE.object    [ ("_type", JE.string "Flag")
                            , ("x_position", JE.int w)
                            , ("y_position", JE.int h)
                            ]
        str = JE.encode 0 json
    in
        toBackEnd str

sendClick: Int -> Int -> Cmd Msg
sendClick h w =
    let 
        json = JE.object    [ ("_type", JE.string "Expose")
                            , ("x_position", JE.int w)
                            , ("y_position", JE.int h)
                            ]
        str = JE.encode 0 json
    in
        toBackEnd str

sendQuit: Cmd Msg
sendQuit =
    let 
        json = JE.object    [ ("_type", JE.string "Quit")
                            ]
        str = JE.encode 0 json
    in
        toBackEnd str

requestNewField: Model -> Cmd Msg
requestNewField model =
    let 
        json = JE.object    [ ("_type", JE.string "Start")
                            , ("width", JE.int model.width)
                            , ("height", JE.int model.height)
                            , ("num_bombs", JE.int model.num_bombs)
                            ]
        str = JE.encode 0 json
    in
        toBackEnd str


type Msg = Error String
        | UpdateWidth String
        | UpdateHeight String
        | UpdateNumBombs String
        | UpdateState GameState
        | UpdateField (List (List Tile))

        | Quit

        | RequestNewField
        | Click Int Int
        | Flag Int Int
        | Unflag Int Int

port toBackEnd : String -> Cmd msg
port toFrontEnd : (JE.Value -> msg) -> Sub msg

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        UpdateWidth newWidth ->
            ( {model| width = newWidth |> String.toInt |> Maybe.withDefault 10 |> max 1 }, Cmd.none)
        UpdateHeight newHeight ->
            ( {model| height = newHeight |> String.toInt |> Maybe.withDefault 10 |> max 1 }, Cmd.none)
        UpdateNumBombs newNumBombs ->
            ( {model| num_bombs = newNumBombs |> String.toInt |> Maybe.withDefault 8 |> max 1 }, Cmd.none)
        UpdateState newState ->
            ( {model| state = newState}, Cmd.none)
        UpdateField newField ->
            ( {model| field = newField}, Cmd.none)

        Quit ->
            ( model, sendQuit )

        RequestNewField ->
            ( model, requestNewField model )
        Click h w ->
            ( model, sendClick h w)
        Flag h w ->
            ( model, sendFlag h w)
        Unflag h w ->
            ( model, sendUnflag h w)

        Error _ ->
            ( model, Cmd.none)




-- VIEW

-- NOTE: Control+Click on MacOS, also doesn't seem to work that well (it often causes a LeftClick afterwards)
-- Based on https://github.com/dc25/onRightClickElm but it needed to be updated
onRightClick: Msg -> Attribute Msg
onRightClick message =
  custom
    "contextmenu"
    (JD.succeed 
        { message = message
        , stopPropagation = True
        , preventDefault = True
        })

onLeftClick: Msg -> Attribute Msg
onLeftClick =
    Html.Events.onClick 
    
createFieldSlot: Int -> Int -> Tile -> Html Msg
createFieldSlot h w tile =
    if tile.is_bomb then
        td [] [ button [] [ text "B"]]
    else if tile.has_flag then
        td [] [ button [onRightClick (Unflag h w)] [ text "F"]]
    else if not tile.is_shown then
        td [] [ button [onLeftClick (Click h w), onRightClick (Flag h w)] [ text "?"]]
    else if not tile.was_clicked then
        td [] [ button [onLeftClick (Click h w), onRightClick (Flag h w)] [ text (String.fromInt tile.num_bombs_around)]]
    else 
        if tile.num_bombs_around > 0 then
            td [] [ text (String.fromInt tile.num_bombs_around)]
        else 
            td [] [ text " "]


createFieldRow: Int -> (List Tile) -> Html Msg
createFieldRow h tiles =
    tr [] (List.indexedMap (\w -> createFieldSlot h w) tiles)

view : Model -> Html Msg
view model =
  div []
    [ div [] [ label [] [ text "Width:" ]
             , input [ type_ "number", value (String.fromInt model.width), onInput UpdateWidth ] []
             , label [] [ text "Height:" ]
             , input [ value (String.fromInt model.height), onInput UpdateHeight ] []
             , label [] [ text "#of Bombs:" ]
             , input [ value (String.fromInt model.num_bombs), onInput UpdateNumBombs ] []
             , button [ onLeftClick RequestNewField ] [ text "Create!" ]
             , button [ onLeftClick Quit ] [ text "Quit :(" ]
             ]
    , case model.state of
        InProgress -> text ""
        Won -> text "You Won!"
        Loss -> text "You Lost."
    , table [] (List.indexedMap createFieldRow model.field)
    ]

-- SUBSCRIPTIONS

decodeTile: JD.Decoder Tile
decodeTile =
    JD.map5 Tile
        (JD.field "num_bombs_around" JD.int)
        (JD.field "has_flag" JD.bool)
        (JD.field "is_bomb" JD.bool)
        (JD.field "is_shown" JD.bool)
        (JD.field "was_clicked" JD.bool)

decodeField: JD.Decoder (List (List Tile))
decodeField =
    JD.list (JD.list decodeTile)

decodeValue : JE.Value -> Msg
decodeValue raw =
    let
        object_type =
            JD.decodeValue (JD.field "_type" JD.string) raw
    in
        case object_type of
                Ok "NewField" -> 
                    case JD.decodeValue (JD.field "tiles" decodeField) raw of
                        Ok newField ->
                            UpdateField newField
                        Err error ->
                            Error (JD.errorToString error)
                Ok "InProgress" ->
                    UpdateState InProgress
                Ok "Won" ->
                    UpdateState Won
                Ok "Loss" ->
                    UpdateState Loss
                Ok unknown_type ->
                    Error ("Unknown type: "++unknown_type)
                Err error ->
                    Error (JD.errorToString error)


subscriptions : Model -> Sub Msg
subscriptions model =
    toFrontEnd (decodeValue)