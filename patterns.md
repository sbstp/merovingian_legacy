# Directory structure patterns
Paths that start with `/` indicate that they start at the root of the import folder.

## TV

### Single episode
```
/episode.mkv

/episode/episode.mkv
```

### Season
```
/season/episode1.mkv
/season/episode2.mkv
/season/episodeN.mkv
```

### Series
```
/series/episode1.mkv
/series/episode2.mkv
/series/episodeN.mkv

/series/episode1/episode1.mkv
/series/episode2/episode2.mkv
/series/episodeN/episodeN.mkv

/series/season1/episode1.mkv
/series/season1/episodeN.mkv
/series/season2/episdoe1.mkv
/series/season2/episodeN.mkv
/series/seasonN/episode1.mkv
/series/seasonN/episodeN.mkv

/series/season1/episode1/episode1.mkv
/series/season1/episodeN/episodeN.mkv
/series/season2/episode1/episode1.mkv
/series/season2/episodeN/episodeN.mkv
/series/seasonN/episode1/episode1.mkv
/series/seasonN/episodeN/episodeN.mkv
```

## Movie

### Single movie
```
/movie.mkv

/movie/movie.mkv
```

### Movie pack
```
/moviepack/movie1.mkv
/moviepack/movie2.mkv
/moviepack/movieN.mkv

/moviepack/movie1/movie1.mkv
/moviepack/movie2/movie2.mkv
/moviepack/movieN/movieN.mkv
```

## Subtitles
Episode and movie are interchangeable in this section.

### Subtitle without language
```
episode.mkv
episode.srt
```

### Subtitle with language
```
episode.en.mkv
episode.fr.srt

episode.eng.mkv
episode.fre.srt
```

### Subtitle in subdirectory
This pattern is only relevant when there's a single episode or movie in the folder.

```
episode.mkv
subs/english.srt

episode.mkv
subtitles/english.srt
```

```
episode1.mkv
episode2.mkv
episodeN.mkv

subs/episode1.srt
subs/episode2.srt
subs/episodeN.srt
```
