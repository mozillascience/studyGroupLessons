#Scheduling in the Bash Shell

 - **Authors**: Susan Allen
 - **Research field**: Earth & Ocean Science
 - **Lesson Topic**: Scheduling in the Bash Shell

## Scripting Review

`echo` prints a string to the command line:

```
> echo "Time to go to class"
```

`say` says a phrase out loud (mac only?)

```
> say "Time to go to class"
```

We can combine shell commands by plasing them in a text file called `talk.sh`:

```
echo "Time to go to class"
say "Time to go to class"
```

and re-run those commands as a script:

```
> bash ./talk.sh
```

We can make the file executable using `chmod`:

```
> chmod 700 talk.sh
```

The first digit of the 700, the 7, makes the file read, write and executable by me, the owner.  The second digit, a 0, makes the files not readable, writeable or executable by my group and the third digit 0, does the same to everyone else.  This is way overkill here.  Better would be:

```
> chmod 755 talk.sh
```

Which allows everyone else read and execute permission but not write.  Read = 4, Write = 2 and Execute = 1, add them to get the digit.

Then, once the file is executable, I can just run it:

```
> ./talk.sh
```

## `at`

The `at` command lets us run a bash command at a later time, instead of immediately:

```
> at -f ./talk.sh now +1 minute
```

This command adds (to the `at` queue) `talk.sh` to be run 1 minute from now.  `-f` means I'll have my commands in a file, not type them out on the command line.  In addition to specifying time as an increment you can also specify it in various ways.  For example:

```
> at -f ./talk.sh 6pm today
```

or

```
> at -f ./talk.sh 18:00
```

This works as is on some Linux machines.  To get it work on my mac, I had to activate the `at` queue with this command:

```
>sudo launchctl load -w /System/Library/LaunchDaemons/com.apple.atrun.plist
```

You can see what you have on the `at` queue with `atq`:

```
> atq
```

and then when you know the number (n) of the item you wish to remove from the queue you can remove it using `atrm`:

```
>atrm n
```

The results from the `at` job, ie, the output that would in general go to standard out, end up in the local mail.  You can access the local mail using the command `mail`

```
>mail
```

This is not very useful for a reminder as I never use this mail.  Instead, you can make a script `reminder.sh`

```
SHELL=/bin/bash

recipient="myname@eos.ubc.ca"
subject="Reminder"
echo "Go to class, it's" $(date) | mail -s $subject $recipient
```

The first line is just there to ensure that `at` runs the job with bash. (confirmed works on OSX - might need to set up a mail client on some linux machines).


## Cron

Use cron if you want to run the same job repeatedly.  To setup a cron job you need a script and you need a crontab file.  The easiest way to construct such a file is to type a file and then load it using `crontab`.

Create a file `crontab.txt`:

```
* * * * * /Users/sallen/Desktop/swc/reminder.sh
```

Load it:

```
>crontab crontab.txt
```

Cron then saves this information in the correct place.  Once loaded, you can delete the file if you wish.
To look at your currently loaded `crontab` file use `crontab -l`.  To remove your `crontab` file use `crontab -r`.

With the five stars, this reminder is sent every minute, forever, until I remove it.  More information on correctly setting the five integers is available [here](http://www.thesitewizard.com/general/set-cron-job.shtml), which I found very helpful.

If you want to run many cron jobs, just list them one after another in your `crontab.txt` file before you load it.  Part of an example is shown below.  Lines starting with # are comments.

```
# m h  dom mon dow   command
30 9   *   *   *    /data/user/cronjob.sh
6 0   *   *   *    /data/user/Fraser_flow.cron.sh
#
MEOPAR=/data/user/MEOPAR
NOWCAST_TOOLS=tools/nowcast
0  4 * * *  ${MEOPAR}/${NOWCAST_TOOLS}/weather.cron.sh
```
