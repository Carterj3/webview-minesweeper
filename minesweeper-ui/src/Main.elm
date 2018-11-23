port module Main exposing (main)

import Browser
import Html exposing (Html, button, div, text, input)
import Html.Attributes exposing (type_, value, placeholder)
import Html.Events exposing (onClick, onInput)

import Json.Encode exposing (Value, encode)
import Json.Decode exposing (decodeValue, string)


main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }


-- MODEL

type alias Model = 
    { message: String
    }

init : () -> (Model, Cmd Msg)
init _ =
  ( { message= "Unset" }
  , Cmd.none
  )


-- UPDATE

type Msg = UpdateModel String | Clear

port toBackEnd : String -> Cmd msg
port toFrontEnd : (Value -> msg) -> Sub msg

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        UpdateModel newMessage ->
            ( {model| message= newMessage}, Cmd.none )
        Clear ->
            ( {model| message= "Cleared"}, toBackEnd model.message)


-- VIEW

view : Model -> Html Msg
view model =
  div []
    [ div [] [ input [ placeholder "Text to set", value model.message, onInput UpdateModel ] [] ]
    , div [] [ text model.message ]
    , div [] [ button [ onClick Clear ] [ text "Clear Text" ] ]
    ]

-- SUBSCRIPTIONS

decodeValue : Value -> Msg
decodeValue x =
    UpdateModel (encode 0 x)
    -- let
    --     result =
    --         Decode.decodeValue Decode.string x
    -- in
    --     case result of
    --         Ok string ->
    --             Increment

    --         Err _ ->
    --             Decrement


subscriptions : Model -> Sub Msg
subscriptions model =
    toFrontEnd (decodeValue)