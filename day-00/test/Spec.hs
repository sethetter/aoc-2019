module Spec where

import Test.Hspec
import Lib

run :: IO ()
run = hspec $ do
  describe "myFunc" $ do
    it "reverses the string" $ do
      myFunc "hey dude" `shouldBe` "edud yeh"
