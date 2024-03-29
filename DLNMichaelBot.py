#!/usr/bin/python3

# Michael(DLN) Real Life AI Bot. Just like talking to Michael in real life!
# Written by DasGeek
#
# Licensed under the Mozilla Public License Version 2.0
# Fedora-License-Identifier: MPLv2.0
# SPDX-License-Identifier: MPL-2.0
#
# This is free software.
# For more information on free software, see
# <https://www.gnu.org/philosophy/free-sw.en.html>.
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at <https://mozilla.org/MPL/2.0/>.
#

import time #to put sleep to make AI look intelligent
from os import system

def intro():
    print('\n')
    print ("Hello my name is like Michael, I'm a 'like' AI chat bot but programmed 'like' to talk just like the real Michael.\n")
    time.sleep(1)
    print("My favorite thing's are 'like' stools, blow drying my hair, playing with 'like' Mr. Potato heads, and Linux.\n")
    time.sleep(2)
    print("I also love playing video games that have 'snipey' wifles in them.")
    time.sleep(1)
    print("Don't tell anyone but I have very weak thumbs and you know what makes my thumbs stronger? ACTIVE SITTING!\n")
    time.sleep(2)
    print("Before we go any further, I would just like to let you know that I've used Linux for 20 years.\n")
    time.sleep(2)
    print("Well, like, um, like, open-source is cool but I don't like to open-source my OBS scenes because like they're like,")
    print("a secret source...haha get it...I said source instead of sauce! That's my favorite dad joke!\n")
    time.sleep(3)
    print("Look at me just going on and on, that's so rude of me")

def name():
    username = input("What's your name?: ")
    print('\n')
    print("Nice to meet you " + username + "!\n")


def distro():
    #Get the name of the players favorite distro
    distro = input("What's like your favorite like distro? [Enter it here]: ")
    print('\n')
    time.sleep(2)
    print("Oh so you like " + distro + ". That's an ok distro but I prefer to use Rebecca Black Linux because I'm a hipster.\n")
    time.sleep(2)
    print("Is it Friday? Cause I love gettin' down on Friday!\n")
    time.sleep(2)

def age():
    # Error check for integer
    while True:
        try:
            age_quest = input("How old are you anyways?: ") #Get users age
            turn_old = 100 - int(age_quest) #Calcuate how many years until they turn 100
            time.sleep(2)
            print("Wow you will turn 100 in", turn_old, "years!")
            time.sleep(2)
            print("I'm full of really interesting facts like that!\n")
            break;
        except ValueError:
            print("Please enter an actual integer...number...not text...seriously.")
        except ValueError:
            break;
    time.sleep(2)

def madlib():
    print("Do you like Mad Libs? I love them, let's play one.")
    time.sleep(2)
    print("I'm really spontaneous and fun like that. Ok so here we go...\n")
    time.sleep(2)
    #Michael AI Mad Lib Section
    #Get series of questions for inputting into madlib
    obj_name = input("Give me the name of an object in the room (example: table): ")
    food_name = input("What's your favorite food?: ")
    color_name = input("What's your favorite color: ")
    time.sleep(1)
    print("Wow so you like " + color_name + "? That's cute, my favorite color is Clear! Now you know more about me!")
    anim_name = input("What's your favorite animal?: ")
    print("Ok, using my advanced AI de-sequencer I've calculated a Madlib for you\n")
    time.sleep(3)
    print("............De-sequencing...flushing daemons............\n")
    time.sleep(2)
    print("\n")
    print("Madlib By Michael AI")
    print("The [" + anim_name +"] jumped onto the [Stool] to practice active sitting. Afterwards, the [" + anim_name + "] decided to eat some [" + food_name + "],\nwhile staring at an OBS scene that was [" + color_name + "]." )
    print("\n")
    time.sleep(5)
    print("Did you see what I did there? I don't care what you picked as your object name, it was replaced with Stool!\n")
    time.sleep(3)
    print("Ok...so now that I've learned all that interesting info from you. I will pass it on to Google and Facebook\n")
    time.sleep(5)

def designer():
    print("Did you know I'm also a graphic designer and marketer? I will draw you something\n")
    time.sleep(4)
    print("\n")
    #draw a stool
    print("*******************")
    print("  ***************  ")
    print("  **           **  ")
    print("  **           **  ")
    print("  **           **  ")
    print("  **           **  ")
    print("  **           **  ")
    print("  **           **  ")
    print("  **           **  ")
    print("\n")
    print("It's a stool for active sitting!")
    time.sleep(2)
    print("That was fun!")

def icanhazdadjoke():
    # Get a random bad joke. API maintainers ask for the specific tool name in User-Agent to help watch for abuse.
    import ssl
    import urllib.request
    import urllib.error
    url = 'https://icanhazdadjoke.com'
    headers = {
        'Accept': 'text/plain',
        'User-Agent': 'Michael-AI (https://github.com/dasgeekchannel/MichaelAIChatBot)'
        }
    time.sleep(2)
    print('\nOh, hey! Remember how I told that joke earlier?\n')
    time.sleep(1)
    print('Well I got pelenty of them!\n')
    time.sleep(2)
    print('Oh, I just remembered this one....')
    time.sleep(2)
    wantsbadjoke = True
    while wantsbadjoke:
        try:
            ssl._create_default_https_context = ssl._create_unverified_context
            req = urllib.request.Request(url, headers=headers)
            with urllib.request.urlopen(req) as response:
                canihazjoke = response.read().decode()
            print('\n' + canihazjoke + '\n')
            print("Ha, that was a lot of fun, wasn't it?\n")
            time.sleep(2)
            print("I could go all day! Want to hear another one?\n")
            keepupthetorture = input('[y/n]: ')
            if keepupthetorture in ['Y', 'y']:
                print("\nAlright, let's see...\n\nOh! Here's one.")
                time.sleep(2)
                wantsbadjoke = True
            elif keepupthetorture in ['N', 'n']:
                print("\nOkay, but it's your loss!\n")
                wantsbadjoke = False
            elif keepupthetorture not in ['Y', 'y', 'N', 'n']:
                print("\nWell, you didn't answer with a 'y' or a 'n' so I am just going to give you another piece of gold!\n")
                time.sleep(2)
                print('Here goes...')
                time.sleep(1)
                wantsbadjoke = True
        except urllib.error.HTTPError as e:
            escode = str(e)
            e301 = '301: Moved Permanently'
            e307 = '307: Temporary Redirect'
            e400 = '400: Bad Request'
            e401 = '401: Unauthorized'
            e403 = '403: Forbidden'
            e404 = '404: Not Found'
            e408 = '408: Request Time-out'
            e500 = '500: Internal Server Error'
            e501 = '501: Not Implemented'
            e502 = '502: Bad Gateway'
            e503 = '503: Service Unavailable'
            e504 = '504: Gateway Time-out'
            e505 = '505: TTP Version not supported'
            if escode in [e301, e307, e400, e401, e403, e404, e408, e500, e501, e502, e503, e504, e505]:
                error_print = escode.split(': ')
                print('Aw man, I got an error code. I think it was ' + error_print[0])
                print('I think that means ' + error_print[1] + 'but who knows. Maybe next time...')
                wantsbadjoke = False

def lastgame():
    time.sleep(2)
    print("Ok one more game")
    time.sleep(2)
    print("Let's make a song!")
    time.sleep(2)
    #song input
    pluralr = input("Type something plural that is red. Example roses: ")
    pluralb = input("Type something plural that is blue. Example oceans: ")
    plurall = input("Type something plural that you love. Example distros: ")
    verb1 = input("Enter a verb. Example: running: ")
    print("\n")
    print("Generating a song for you. Did you know I play the recorder?\n")
    time.sleep(2)
    print("generating.\n")
    time.sleep(1)
    print("generating..\n")
    time.sleep(1)
    print("generating...\n")
    time.sleep(1)
    print(pluralr + " are red.")
    time.sleep(2)
    print(pluralb + " are blue.")
    time.sleep(2)
    print("I like " + plurall + ".")
    time.sleep(2)
    print("But not as much as " + verb1 + " with you!")
    time.sleep(2)
    print("\n")

def muffincakes():
    print("Oh shoot, I forgt one thing!\n")
    time.sleep(2)
    muffincakes = input("Muffins or Cupcakes?: ")
    while muffincakes not in ["Cupcakes", "cupcakes", "Cupcake", "cupcake", "Muffin", "muffin", "Muffins", "muffins"]:
        print("No, no, no, no, no, no, no, no.......That is not what I asked!")
        muffincakes = input("Muffins or Cupcakes?: ")
    if muffincakes in ["Muffin", "muffin", "Muffins", "muffins"]:
        print("Thank you.")
        time.sleep(1)
        print(".")
        time.sleep(1)
        print(".")
        time.sleep(1)
        print(".")
        time.sleep(1)
        print(".\nVINDICATION!\n")
    elif muffincakes in ["Cupcakes", "cupcakes", "Cupcake", "cupcake"]:
        print("Ryan, is that you?")
        time.sleep(1)
        print("J/K, I know it is!\nWho else is going to choose dumbcakes!\n\n")
    time.sleep(3)

def ubuntusummit():
    print("Did you know that in 2022 I gave a talk at the Ubuntu Summit?")
    time.sleep(1)
    print("Ryan almost made me miss it, getting me sick and all, but whatever. He can't hold me back!")
    time.sleep(1)
    print("")
    print("If you would like to watch the talk I gave at the Ubuntu Summit, you can watch it on YouTube!")
    time.sleep(1)
    print("The link is: https://www.youtube.com/watch?v=D2TKlxdpmus")
    time.sleep(1)
    print(" ")
    print("You can also just go to YouTube and search for 'Ubuntu Summit 2022 | Open Source Marketing Done Right'")
    print(" ")
    time.sleep(2)

def fin():
    print("Wow, look at the time. This has been so much fun. Thanks for talking with me!\n")
    print("If you want to support the show, go to dlnstore.com and buy yourself a Linux Is Everywhere T-short.\n")
    print("Remember, the journey itself, is just as important as the Destination!\n")
    time.sleep(1)
    print("Goodbye!")
    quit(0)

def init():
    system('clear')
    print('''
    ___       ___       ___       ___       ___       ___       ___            ___       ___   
   /\__\     /\  \     /\  \     /\__\     /\  \     /\  \     /\__\          /\  \     /\  \  
  /::L_L_   _\:\  \   /::\  \   /:/__/_   /::\  \   /::\  \   /:/  /         /::\  \   _\:\  \ 
 /:/L:\__\ /\/::\__\ /:/\:\__\ /::\/\__\ /::\:\__\ /::\:\__\ /:/__/         /::\:\__\ /\/::\__|
 \/_/:/  / \::/\/__/ \:\ \/__/ \/\::/  / \/\::/  / \:\:\/  / \:\  \         \/\::/  / \::/\/__/
   /:/  /   \:\__\    \:\__\     /:/  /    /:/  /   \:\/  /   \:\__\          /:/  /   \:\__\  
   \/__/     \/__/     \/__/     \/__/     \/__/     \/__/     \/__/          \/__/     \/__/  
    ''')
    intro()
    name()
    distro()
    age()
    madlib()
    designer()
    icanhazdadjoke()
    lastgame()
    muffincakes()
    ubuntusummit()
    fin()

if __name__ == "__main__":
    init()
