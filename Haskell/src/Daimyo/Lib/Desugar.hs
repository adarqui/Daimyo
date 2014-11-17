module Daimyo.Lib.Desugar (
) where

list_comp = [ (x,y,z) | x<-[1..10], y<-[11..20], z<-[21..30]]
list_comp' = [1..10] >>= \x -> [11..20] >>= \y -> [21..30] >>= \z -> [(x,y,z)]

t_list_comp = list_comp == list_comp'
