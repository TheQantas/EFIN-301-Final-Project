library(jsonlite)

disagree <- NULL

investment <- 0
count <- 0
returns <- 0
min_diff <- 0.3

for (i in 1:nrow(dynamic)) {
  game <- dynamic[i,]
  lines <- odds[i,]
  margin <- fromJSON(game$margin_distr)
  away_wins <- margin[1:-game$margin_min]
  home_wins <- margin[(-game$margin_min+1):length(margin)]
  away_prob <- sum(away_wins)
  home_prob <- sum(home_wins)
  
  away_infer = 1/lines$away_odds / (1/lines$away_odds+1/lines$home_odds)
  home_infer = 1/lines$home_odds / (1/lines$away_odds+1/lines$home_odds)
  
  away_diff <- away_prob - away_infer
  home_diff <- home_prob - home_infer
  
  # done = F
  
  if (away_diff > min_diff) {
    # done = T
    count <- count + 1
    scalar <- away_diff
    investment <- investment + scalar
    if (lines$away_score > lines$home_score) {
      returns <- returns + scalar * lines$away_odds
    }
  }
  if (home_diff > min_diff) {
    # done = T
    count <- count + 1
    scalar <- home_diff
    investment <- investment + scalar
    if (lines$away_score < lines$home_score) {
      returns <- returns + scalar * lines$home_odds
    }
  }
  
  # if (!done) {
  #   print(c(away_diff,home_diff,1/lines$away_odds,1/lines$home_odds,away_prob,home_prob))
  # }
  
  disagree <- append( away_diff , disagree )
  disagree <- append( home_diff , disagree )
}


# print(mean(disagree))
# print(sd(disagree))
# print(min(disagree))


print(returns/investment-1)
print(count/nrow(dynamic))


