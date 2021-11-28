# Chesshound

Chesshound is a rust library and cli tool that is intended for analyzing player patterns within any set of games. Its main goals include:

- "At a glance" analysis of player mistakes through the use of opening statistics and blunder patterns.
- Easy scraping of player games from chess.com or lichess.org based on useful criteria such as time period.
- Comparison of user statistics to players at a similar rating level.
- Preprocessing of game data such as annotating with engine analysis.
- Identification of the most important areas of improvement for rating increases.

Thus, the currently planned features are:

- [x] Opening explorer that works with arbitrary sets of games, rather than all games within the account as with chess.com
- [ ] An accessible wrapper around the chess.com and lichess APIs for gathering player game data
- [ ] Sampling functions that allow the user to collect data from a wider variety of players
- [ ] Functions to take useful statistics on sets of games, such as win rate, average accuracy, etc.
- [ ] An API to compare user statistics with players of similar rating level, and integration of this API within the Chesshound CLI.
- [ ] A rating forecaster to let users know where they currently stand against players at their level.
- [ ] Engine annotation of games, and the ability to turn annotated game sets into analysis friendly formats such as CSVs.
- [ ] Rating prediction with recently played games as input.
- [ ] Annotating engine decisions with explanations.
- [ ] Causal modeling between explained engine decisions and rating.

Farther down the road is a goal to make the application more accessible by:

- Creating a python wrapper around library functionality.
- Using the python library to implement a GraphQL service for the application.
- Designing a ReactJS frontend for end users who wish to analyze their games or use the service to extract game data.

## Contributing

The project will be open to contributions once it has reached the 1.0.0 release.
