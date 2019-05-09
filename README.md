# Complete Tracker
This goal of this application is to let you track your completion of pretty much anything. It could be TV Shows, books, movies, anime or games.

Monopolizing this information is a complete non-goal, as it should be open and usable (like Wikipedia).

This project is an application that uses a web interface, so you should spin up your own and use it however you like.

Although this helps with a single person's completion tracking, it also handles series' information compilation. If there is a cinematic universe or book series that is gigantic, I want this application to be a reliable font of information to see how everything's connected.

Multiple people can use the completion tracking at the same time. It would be recommended to do this only with other people you know well, as there is no login system.

## Running the Program
To run the program, you will need to have Rust installed.
TODO

## Jargon
There are a bunch of terms that are used throughout, so their meaning is clearly identified here.

**Arc** - A grouping of sources within a series because they have related story elements that are different than other sources not in the same Arc. They are also directly related. (eg. Ezio's 3 games in Assassin's Creed is an arc,and each one leads into the next, but the first 4 games are focused on Desmond's series)

**Series** - A collection of sources that are related, somehow. It could be the setting, gameplay, or characters. The titles are usually similar. (eg. Assassin's Creed has many games, but not all of them are directly related)

**Source** - The original source material (original work) like a movie, game, or book. (eg. A single Assassin's Creed game)

**Universe** - A collection of series that are related because they all happen in the same (artificial) world. The series in the universe will be disjointed but play on the same basic principles (eg. Brandon Sanderson's Cosmere, or Marvel)

## Project Layout
All information pertaining to the sources will be in the JSON file format.

This will ideally be created within the app, and then written out to these JSON files.

### Run Info
This is info that needs to be specified for the server to run. Anything optional will have the `Option` type. The full specification can be found in the `tracking.rs` file in the RunInfo struct.

When specifying the tracking info, only a single tracker may leave the `tracking_file` field null.

A sample is given here: TODO

### Sources
This is the folder that holds the information on the source material. This could be books, games, movies.

It holds data about each source, like it's original release date for different regions, author, summary, etc.

This application should ideally be able to load everything on startup, keeping everything in memory for each request.

### Series
This holds the information on each series, such as related sources and ordering of sources. This is kept separate from the actual sources so that it doesn't become a web of nightmares to keep track of everything.

## Site Data
To support the open-ness of the application, any user should be able to download their data into a format that would be easy to use or import elsewhere (JSON or CSV).

We should be able to re-import user information

It should be optional to download the information related to the sources.
