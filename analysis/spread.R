library(jsonlite)

disagree <- NULL

investment <- 0
count <- 0
returns <- 0
min_diff <- 0

for (i in 1:nrow(dynamic)) {
  game <- dynamic[i,]
  lines <- odds[i,]
  margin <- fromJSON(game$ou_distr)
  divider <- lines$home_line - game$margin_min
  under_wins <- margin[1:divider]
  over_wins <- margin[divider:length(margin)]
  under_prob <- sum(under_wins)
  over_prob <- sum(over_wins)
  
  under_infer = 0.5
  over_infer = 0.5
  
  under_diff <- under_prob - under_infer
  over_diff <- over_prob - over_infer
  
  # done = F
  
  if (under_diff > min_diff) {
    # done = T
    count <- count + 1
    scalar <- under_diff
    investment <- investment + scalar
    if (lines$home_score - lines$away_score < lines$home_line) {
      returns <- returns + scalar * 1.91
    }
  }
  if (over_diff > min_diff) {
    # done = T
    count <- count + 1
    scalar <- over_diff
    investment <- investment + scalar
    if (lines$home_score - lines$away_score > lines$home_line) {
      returns <- returns + scalar * 1.91
    }
  }
  
  # if (!done) {
  #   print(c(under_diff,over_diff,1/lines$under_odds,1/lines$over_odds,under_prob,over_prob))
  # }
  
  disagree <- append( under_diff , disagree )
  disagree <- append( over_diff , disagree )
}


# print(mean(disagree))
# print(sd(disagree))
# print(max(disagree))


print(returns/investment-1)
print(count/nrow(dynamic))


