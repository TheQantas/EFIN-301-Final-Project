library(ggplot2)

# main_theme <- theme(
#   plot.title = element_text(color = "white", size = 14),
#   axis.title.x = element_text(color = "white", size = 12),
#   axis.text.x = element_text(color = "white", size = 12),
#   axis.title.y = element_text(color = "white", size = 12),
#   axis.text.y = element_text(color = "white", size = 12),
#   axis.ticks = element_line(color = "transparent"),
#   axis.line = element_line(color = "grey"),
#   plot.background = element_rect(fill = "transparent",color=NA),
#   panel.background = element_rect(fill = "transparent",color=NA),
#   panel.grid.major = element_line(color = "grey"),
#   panel.grid.minor = element_line(color = "transparent")
# )

ggplot(data=offense_by_year,aes(x=year,y=total/game_count)) +
  geom_point(color="black") +
  geom_line(color="black") +
  geom_smooth(method="lm",color="blue",se=F) +
  labs(
    title="Drives per Team per Game by Season",
    y="Drives",
    x="Season"
  )

ggplot(data=offense_by_year,aes(x=year,y=touchdown/game_count)) +
  geom_point(color="black") +
  geom_line(color="black") +
  geom_smooth(method="lm",color="blue",se=F) +
  labs(
    title="Touchdowns per Team per Game by Season",
    y="Touchdowns",
    x="Season"
  )

ggplot(data=offense_by_year,aes(x=year,y=field_goal/game_count)) +
  geom_point(color="black") +
  geom_line(color="black") +
  geom_smooth(method="lm",color="blue",se=F) +
  labs(
    title="Field Goals per Team per Game by Season",
    y="Field Goals",
    x="Season"
  )

ggplot(data=offense_by_year,aes(x=year,y=punt/game_count)) +
  geom_point(color="black") +
  geom_line(color="black") +
  geom_smooth(method="lm",color="blue",se=F) +
  labs(
    title="Punts per Team per Game by Season",
    y="Punts",
    x="Season"
  )

# ggplot(data=offense_by_year,aes(x=year,y=safety/game_count)) +
  # geom_point(color="black") +
  # geom_line(color="black") +
  # geom_smooth(method="lm",color="blue",se=F) +
  # labs(
  #   title="Safeties per Team per Game by Season",
  #   y="Safeties",
  #   x="Season"
  # ) + main_theme
  