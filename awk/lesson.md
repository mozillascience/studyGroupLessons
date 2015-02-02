#Text Parsing with `awk`, `sed` and `paste`

 - **Authors**: Kathi Unglert
 - **Research field**: Volcano Geophysics
 - **Lesson Topic**: `awk`, `sed` and `paste` in the bash shell.

## `awk`

Check whether you have `awk` installed by using the `which` command:

```
$: which awk
```

You should get something like this:

```
/usr/bin/awk
```

If you don't have `awk` installed you can get it [here](http://linux.softpedia.com/get/Text-Editing-Processing/Filters/GNU-awk-207.shtml).


`volcano_observations.dat` contains the following lines
(try viewing it with `cat volcano_observations.dat`):

```
month level observation
april 1 steam
may 2 ash
june 4 plinian
july 1 steam
august 2 earthquakes
september 3 subplinian
october 2 earthquakes
november 1 steam
```

We can achieve the same effect as `cat` with:

```
awk '{print $0}' volcano_observations.dat
```

  - **Challenge Question**: How to get output to print to file?
  - **Solution**: `awk '{print $0}' volcano_observations.dat > observations2.dat`


We can select individual columns to print like this:

```
awk '{print $3}' volcano_observations.dat
```

But, maybe we don't want the header line `observation` in our output.
We can get rid of it via

```
awk '{if (NR > 1) print $3}' volcano_observations.dat
```

We can pick all entries that contain "earth" in their observation column:

```
awk '{if (index($3, "earth") != 0) print $3}' volcano_observations.dat
```

Or, we can combine if statements to pick all entries that have an "e"
in their observation column, not including the header:

```
awk '{if (NR > 1 && index($3, "e") != 0) print $3}' volcano_observations.dat
```

Or, the same thing, but print out the whole entry:

```
awk '{if (NR > 1 && index($3, "e") != 0) print $0}' volcano_observations.dat
```

 - **Challenge Question**: How to print all months with alert level of 2 or higher?
 - **Solution**: `awk '{if (NR > 1 && $2 >= 2) print $1}' volcano_observations.dat`


`earthquakes.dat` contains the following (view with `cat`):

```
date time magnitude lat lon depth
2011/03/01 20:21:11.11 3.1 49.12 -123.10 45
2011/03/01 21:45:51.04 3.8 49.21 -123.08 42
2011/03/01 21:53:42.33 2.5 49.01 -122.89  5
2011/03/01 21:58:32.17 3.4 48.99 -122.89  2
2011/03/01 22:03:56.10 4.3 49.03 -123.12 35
2011/03/01 23:22:45.89 3.1 49.01 -122.91  1
2011/03/02 04:17:03.77 4.3 49.02 -123.01 0.5
2011/03/02 12:01:34.32 3.7 49.17 -123.20 43
2011/03/02 15:34:56.51 2.8 49.14 -123.00 46
2011/03/03 05:21:23.09 3.4 49.09 -123.11 41
2011/03/03 08:54:56.67 3.3 49.09 -123.10 43
2011/03/03 16:32:45.52 4.1 49.10 -123.12 42
```

In this data, shallow earthquakes may indicate volcanic activity,
while deep earthquakes may originate from a subduction zone.

 - **Challenge Question**: How to print all earthquake entries from volcano?
 - **Solution**: `awk '{ if (NR > 1 && $6 <= 10) print $0}' earthquakes.dat`

Or, print out only the magnitudes of those:

```
awk '{ if (NR > 1 && $6 <= 10) print $3}' earthquakes.dat
```

Or, the magnitudes and depths:

```
awk '{ if (NR > 1 && $6 <= 10) print $3,$6}' earthquakes.dat
```

We can also add some text to what we return:

```
awk '{ if (NR > 1 && $6 <= 10) print "magnitude",$3,"depth",$6}' earthquakes.dat
```

We'd like to be able to extract which days had volcanic earthquakes.
But, the year, month and day are seaparated by `/` instead of spaces,
so they appear as one column. We can define a new 'field separator'
with the `-F` flag; we can get the month like this:

```
awk -F/ '{ if (NR > 1) print $2}' earthquakes.dat
```

But if we ask for the next column in order to get the days,
we get the whole rest of the line, too - `awk` is no longer
breaking on spaces:

```
awk -F/ '{ if (NR > 1) print $3}' earthquakes.dat
```

We can define additional field separators:

```
awk '{FS="[/ :]"}{if (NR > 1) print $3}' earthquakes.dat
```

Which lets us find the days with volcanic earthquakes:

```
awk '{FS="[/ :]"}{if (NR > 1 && $10 <= 10) print $3}' earthquakes.dat
```

and remove duplicates in the list using `awk`:

```
awk '{FS="[/ :]"}{if (NR > 1 && $10 <= 10) print $3}' earthquakes.dat | awk '!_[$0]++'
```

compare this to:

```
awk '{FS="[/ :]"}{if (NR > 1 && $10 <= 10) print $3}' earthquakes.dat | uniq
```

## `sed`

`sed` is a powerful text manipulation tool,
most usually used for doing find-and-replace operations.

We can substitute all instances of `2011` for `2010` in `earthquakes.dat` (/g = globally):

```
sed 's/2011/2010/g' earthquakes.dat > earthquakes2.dat
```

If we want to substitute the month, we have to be a bit more careful;
replacing every instance of `03` might lead to some unintended replacements.
We can be sure that we're looking at a month if the text follows the pattern `/03/`,
but now `sed` will get confused as to which backslashes are part of the command,
and which backslashes are characters to be replaced.
We have to 'escape' the slashes we want to count as text,
using the character `\`:

```
sed 's/\/03\//\/04\//g' earthquakes.dat > earthquakes2.dat
```

Since this is a bit hard to read,
many implementations of `sed` accept other delimiting characters;
for example, try:

```
sed 's|/03/|/04/|g' earthquakes.dat > earthquakes2.dat
```

We can also substitute inside the original file:

```
sed -i 's/2011/2010/g' earthquakes2.dat
```

or on Mac:

```
sed -i.tmp 's/2011/2010/g' earthquakes2.dat
```

or:

```
sed -i '' 's/2011/2010/g' earthquakes2.dat
```

We can also delete lines:

```
sed '/2011\/03\/03/d' earthquakes.dat > earthquakes2.dat
```

## `paste`

We can combine the columns of two files with the `paste` command:

```
paste -d" " earthquakes.dat earthquakes2.dat > alleqs.dat
```
