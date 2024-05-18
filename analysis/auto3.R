auto3 <- lm(sqrt(td_per_drive)~sqrt(lag_td_per_drive)+year,data=defense) %>% summary()
print(auto3)

auto3f <- lm(sqrt(fg_per_drive)~sqrt(lag_fg_per_drive)+lag_playoff+year,data=offense) %>% summary()
print(auto3f)

auto3fy <- lm(sqrt(fg_per_drive)~lag_playoff+year,data=defense) %>% summary()
print(auto3fy)