library(nortest)

ad.test( offense$field_goal )
ad.test( log(offense$fg_per_drive) )
ad.test( sqrt(offense$fg_per_drive) )

ad.test( offense$td_per_game )
ad.test( log(offense$td_per_game) )
ad.test( sqrt(offense$td_per_game) )

ad.test( offense$fg_per_game )
ad.test( log(offense$fg_per_game) )
ad.test( sqrt(offense$fg_per_game) )
ad.test( offense$fg_per_game**0.75 )

hist(offense$td_per_drive)
hist( log(offense$td_per_drive) )
hist( sqrt(offense$td_per_drive) )
hist(offense$fg_per_game)
