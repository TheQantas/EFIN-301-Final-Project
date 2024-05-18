td_model <- summary( lm(I(touchdown/game_count)~year,data=offense_by_year) )
print(td_model)

fg_model <- summary( lm(I(field_goal/game_count)~year,data=offense_by_year) )
print(fg_model)

punt_model <- summary( lm(I(punt/game_count)~year,data=offense_by_year) )
print(punt_model)

safety_model <- summary( lm(I(safety/game_count)~year,data=offense_by_year) )
print(safety_model)