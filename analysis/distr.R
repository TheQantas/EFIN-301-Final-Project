library(ggplot2)

# par(
#   col.main = "white",  # Color of title
#   col.lab = "white",   # Color of axis labels
#   col.axis = "white",  # Color of axis lines
#   fg = "white",        # Color of text
#   bg = "transparent"   # Background color (transparent)
# )

all_scores <- c(static$away_score,static$home_score)
all_scores_mean <- mean(all_scores)
all_scores_sd <- sd(all_scores)

hist(col="skyblue",all_scores,main="Scores",xlab="Points")
# abline(v = all_scores_mean, col = "purple", lwd = 2)
# abline(v = all_scores_mean+all_scores_sd, col = "purple", lwd = 2, lty="dotted")
# abline(v = all_scores_mean-all_scores_sd, col = "purple", lwd = 2, lty="dotted")

all_errors <- c(static$away_err,static$home_err)
all_errors_mean <- mean(all_errors)
all_errors_sd <- sd(all_errors)

hist(col="skyblue",all_errors,main="Error",xlab="Simulated - Actual")
# abline(v = all_errors_mean, col = "purple", lwd = 2)
# abline(v = all_errors_mean+all_errors_sd, col = "purple", lwd = 2, lty="dotted")
# abline(v = all_errors_mean-all_errors_sd, col = "purple", lwd = 2, lty="dotted")

all_dyn_errors <- c(dynamic$away_err,dynamic$home_err)
all_dyn_errors_mean <- mean(all_dyn_errors)
all_dyn_errors_sd <- sd(all_dyn_errors)

hist(col="skyblue",all_dyn_errors,main="Errors (Forecasted)",xlab="Simulated - Actual")
# abline(v = all_dyn_errors_mean, col = "purple", lwd = 2)
# abline(v = all_dyn_errors_mean+all_dyn_errors_sd, col = "purple", lwd = 2, lty="dotted")
# abline(v = all_dyn_errors_mean-all_dyn_errors_sd, col = "purple", lwd = 2, lty="dotted")

