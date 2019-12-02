{-# LANGUAGE OverloadedStrings,DeriveGeneric #-}
module Lib where

fuelNeeded :: [String] -> Int
fuelNeeded = sum . map fuelFromMass . map (\w -> read w :: Int)

fuelFromMass :: Int -> Int
fuelFromMass x = (x `div` 3) - 2
