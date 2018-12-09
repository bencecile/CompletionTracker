# Complete Tracker
This goal of this application is to let you track your completion of pretty much anything. It could be TV Shows, books, movies, even anime or games.

This project is a web server, so you can spin up your own and use it however you like. Monopolizing this information is a complete non-goal, as it should be open and usable (like Wikipedia).

Although this helps with user completion tracking, it also handles series' information compilation. If there is a cinematic universe or book series that is gigantic, I want this application to be a reliable font of information to see how everything's connected.

## Jargon
There are a bunch of terms that are used throughout, so their meaning is clearly identified here.

**Arc** - A grouping of sources within a series because they have related story elements that are different than other sources not in the same Arc. (eg. Ezio's 3 games in Assassin's Creed is an arc)

**Series** - A collection of sources that are related, somehow. The titles are usually similar. (eg. Assassin's Creed has many games)

**Source** - The original source material (original work) like a movie or game. (eg. A single Assassin's Creed game)

**Universe** - A collection of series that are related because they all happen in the same (artificial) world.

## Project Layout
All information pertaining to the sources will be in the JSON file format.

This will ideally be created within the app, and then written out to these JSON files. Only an admin priveleged user should be able to create the sources.

### Sources
This is the folder that holds the information on the source material. This could be books, games, movies.

It holds data about each source, like it's original release date in different regions, author, summary, etc.

Everything should be sorted and organized logically, so that the program can consume it on startup. (ie. keep series of the same medium together)

### Series
This holds the information on each series, such as related sources and ordering of sources. This is kept separate from the actual sources so that it doesn't become a web of nightmares to keep track of everything.

## Site Data
To support the open-ness of the application, any user should be able to download their data into a format that would be easy to use or import elsewhere (JSON or CSV).

We should be able to re-import user information

It should be optional to download the information related to the sources.
