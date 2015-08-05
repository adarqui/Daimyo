module Pure.UI.Halogen.Todo.Simple (
  uiHalogenTodoSimpleMain
) where

import Prelude
import Data.Tuple
import Data.Maybe
import Data.JSON

import DOM

import Data.DOM.Simple.Document
import Data.DOM.Simple.Element
import Data.DOM.Simple.Types
import Data.DOM.Simple.Window

import Control.Alt
import Control.Bind
import Control.Monad.Eff
import Control.Monad.Eff.Class
import Control.Monad.Eff.Console

import Halogen
import Halogen.Signal
import Halogen.Component

import qualified Halogen.HTML as H
import qualified Halogen.HTML.Attributes as A
import qualified Halogen.HTML.Events as A
import qualified Halogen.HTML.Events.Forms as A
import qualified Halogen.HTML.Events.Handler as E
import qualified Halogen.HTML.Events.Monad as E

import qualified Halogen.HTML.CSS as CSS

import Control.Monad.Aff
import Network.HTTP.Affjax

import Pure.Applications.Todo.Simple

data State = State TodoApp

--type Input = TodoActionRequest
type Input = TodoActionResponse
type Output = TodoActionResponse

appendToBody :: forall eff. HTMLElement -> Eff (dom :: DOM | eff) Unit
appendToBody e = document globalWindow >>= (body >=> flip appendChild e)

-- type HalogenEffects eff = (console :: CONSOLE, ref :: REF, dom :: DOM | eff)
-- type Driver i eff = i -> Eff (HalogenEffects eff) Unit

-- | A `Process` receives inputs and outputs effectful computations which update the DOM.
-- type Process req eff = SF (Tuple req HTMLElement) (Eff (HalogenEffects eff) HTMLElement)

-- type Component m req res = SF1 req (HTML (m res))

-- stateful :: forall s i o. s -> (s -> i -> s) -> SF1 i s
-- stateful' :: forall s i o. s -> (s -> i -> Tuple o s) -> SF i o

ui :: forall eff. Component (E.Event (HalogenEffects (ajax :: AJAX | eff))) Input Input
ui = render <$> stateful (State []) update
  where
  render :: State -> H.HTML (E.Event (HalogenEffects (ajax :: AJAX | eff)) Input)
  render (State v) = H.div_ [layoutHeader, layoutTodos v, layoutButtons]

  update :: State -> Input -> State
  update st RespBusy = st
  update st (RespListTodos xs) = State xs

  layoutHeader   = H.p_ [ H.h1_ [ H.text "Todo MVC" ] ]
  layoutTodos v  = H.p_ [ H.text $ show v]
  layoutButtons  = H.p_ [ H.button [ A.onClick (\_ -> pure handleListTodos) ] [ H.text "list" ] ]

handleListTodos :: forall eff. E.Event (HalogenEffects (ajax :: AJAX | eff)) Input
handleListTodos = E.yield RespBusy `E.andThen` \_ -> E.async compileAff
  where
  compileAff :: Aff (HalogenEffects (ajax :: AJAX | eff)) Output
  compileAff = do
    res <- get "/applications-simple-todos"
    liftEff $ log res.response
    let todos = decode res.response :: Maybe (Array Todo)
    return $ RespListTodos (fromMaybe [] todos)

handleListTodos' :: forall eff. E.Event (HalogenEffects (ajax :: AJAX | eff)) Output
handleListTodos' = E.yield RespBusy `E.andThen` \_ -> E.async compileAff
  where
  compileAff :: Aff (HalogenEffects (ajax :: AJAX | eff)) Output
  compileAff = do
    res <- get "/applications-simple-todos"
    liftEff $ log res.response
    return $ RespListTodos []

uiHalogenTodoSimpleMain = do
  -- runUI :: forall req eff. Component (Event (HalogenEffects eff)) req req -> Eff (HalogenEffects eff) (Tuple HTMLElement (Driver req eff))
  Tuple node driver <- runUI ui
  appendToBody node