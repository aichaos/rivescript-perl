/*
	RiveScript 2 Test Suite -- Designed to demonstrate all the
	functionality that RiveScript 2 is supposed to support.
*/

/******************************************************************************
 * "begin.rs" Command Testing                                                 *
 ******************************************************************************/

> begin
	+ request
	- {ok}
< begin

// Bot Variables
! var name     = RiveScript Test Bot
! var age      = 9000
! var gender   = androgynous
! var location = Cyberspace
! var phone    = 555-1234
! var email    = test@mydomain.com

// Substitutions
! sub +         = plus
! sub -         = minus
! sub /         = divided
! sub *         = times
! sub i'm       = i am
! sub i'd       = i would
! sub i've      = i have
! sub i'll      = i will
! sub don't     = do not
! sub isn't     = is not
! sub you'd     = you would
! sub you're    = you are
! sub you've    = you have
! sub you'll    = you will
! sub he'd      = he would
! sub he's      = he is
! sub he'll     = he will
! sub she'd     = she would
! sub she's     = she is
! sub she'll    = she will
! sub they'd    = they would
! sub they're   = they are
! sub they've   = they have
! sub they'll   = they will
! sub we'd      = we would
! sub we're     = we are
! sub we've     = we have
! sub we'll     = we will
! sub whats     = what is
! sub what's    = what is
! sub what're   = what are
! sub what've   = what have
! sub what'll   = what will
! sub can't     = can not
! sub whos      = who is
! sub who's     = who is
! sub who'd     = who would
! sub who'll    = who will
! sub don't     = do not
! sub didn't    = did not
! sub it's      = it is
! sub could've  = could have
! sub couldn't  = could not
! sub should've = should have
! sub shouldn't = should not
! sub would've  = would have
! sub wouldn't  = would not
! sub when's    = when is
! sub when're   = when are
! sub when'd    = when did
! sub y         = why
! sub u         = you
! sub ur        = your
! sub r         = are
! sub im        = i am
! sub wat       = what
! sub wats      = what is
! sub ohh       = oh
! sub becuse    = because
! sub becasue   = because
! sub becuase   = because
! sub practise  = practice
! sub its a     = it is a
! sub fav       = favorite
! sub fave      = favorite
! sub iam       = i am
! sub realy     = really
! sub iamusing  = i am using
! sub amleaving = am leaving
! sub yuo       = you
! sub youre     = you are
! sub didnt     = did not
! sub ain't     = is not
! sub aint      = is not
! sub wanna     = want to
! sub brb       = be right back
! sub bbl       = be back later
! sub gtg       = got to go
! sub g2g       = got to go

// Person substitutions
! person i am    = you are
! person you are = I am
! person i'm     = you're
! person you're  = I'm
! person my      = your
! person your    = my
! person you     = I
! person i       = you

// Arrays
! array colors = red green blue cyan yellow magenta white orange brown black
  ^ gray grey fuchsia maroon burgundy lime navy aqua gold silver copper bronze
  ^ light red|light green|light blue|light cyan|light yellow|light magenta
! array be     = is are was were

/******************************************************************************
 * Basic Trigger Testing                                                      *
 ******************************************************************************/

/* Atomic Reply
   ------------
   Human says:     hello bot
   Expected reply: Hello human.
*/
+ hello bot
- Hello human.

/* Atomic Reply
   ------------
   Human says:     what is your name
   Expected reply: You can call me RiveScript Test Bot.
*/
+ what is your name
- You can call me <bot name>.

/* Wildcards
   ---------
   Human says:     my favorite thing in the world is programming
   Expected reply: Why do you like programming so much?
*/
+ my favorite thing in the world is *
- Why do you like <star> so much?

/* Wildcards
   ---------
   Human says:     John told me to say hello
   Expected reply: Why would john have told you to say hello?
*/
+ * told me to say *
- Why would <star1> have told you to say <star2>?

/* Wildcards
   ---------
   Human says:     I think the sky is orange.
   Expected reply: Do you think the sky is orange a lot?
*/
+ i think *
- Do you think <star> a lot?

/* Wildcards
   ---------
   Human says:     I am twenty years old
   Expected reply: Tell me that as a number instead of spelled out like "twenty"
   Extra Notes:    When multiple triggers exist that are identical except for
                   their wildcard character, the order of priorities are that
                   _ is always first, # is second, and * last. So in this code
                   and the following one, the "i am # years old" should match
                   if the wildcard is a number and the "i am * years old" should
                   only match otherwise.
*/
+ i am * years old
- Tell me that as a number instead of spelled out like "<star>".

/* Wildcards
   ---------
   Human says:     I am 20 years old
   Expected reply: I will remember that you are 20 years old.
   Extra Notes:    This reply should also set the var "age" to 20 for this user.
*/
+ i am # years old
- <set age=<star>>I will remember that you are <star> years old.

/* Alternations
   ------------
   Human says:     What is your home phone number?
   Expected reply: You can call me at my home number, 555-1234.
   Human says:     What is your office phone number?
   Expected reply: You can call me at my office number, 555-1234.
   Human says:     What is your work phone number?
   Expected reply: You can call me at my work number, 555-1234.
   Human says:     What is your cell phone number?
   Expected reply: You can call me at my cell number, 555-1234.
*/
+ what is your (home|office|work|cell) phone number
- You can call me at my <star> number, <bot phone>.

/* Alternations
   ------------
   Human says:     Are you okay?
                   Are you alright?
                   You okay?
                   You alright?
   Expected reply: I'm fine, thanks for asking.
*/
+ (are you|you) (okay|alright)
- I'm fine, thanks for asking.

/* Optionals
   ---------
   Human says:     How can I contact you?
                   Can I contact you?
   Expected reply: You can have my phone number: 555-1234.
*/
+ [how] can i contact you
- You can have my phone number: <bot phone>.

/* Optionals
   ---------
   Human says:     Do you have an email address?
                   You have an email address?
                   Do you have an email?
                   You have an email?
                   Do you have email?
                   You have email?
   Expected reply: You can e-mail me at test@mydomain.com.
*/
+ [do] you have [an] email [address]
- You can e-mail me at <bot email>.

/* Optionals
   ---------
   Human says:     Tell me your phone number
                   Tell me your number
                   Tell me your home phone number
                   Tell me your home number
                   Tell me your office phone number
                   Tell me your office number
                   Tell me your work phone number
                   Tell me your work number
                   Tell me your cell phone number
                   Tell me your cell number
   Expected reply: My phone number is 555-1234.
*/
+ tell me your [home|office|work|cell] [phone] number
- My phone number is <bot phone>.

/* Arrays
   ------
   Human says:     What color is my blue shirt?
   Expected reply: Your shirt is blue, silly.
   Human says:     What color is my light red shirt?
   Expected reply: Your shirt is light red, silly.
   Human says:     What color is my black shirt?
   Expected reply: Your shirt is black, silly.
*/
+ what color is my (@colors) shirt
- Your shirt is <star>, silly.

/* Arrays
   ------
   Human says:     What color was George Washington's white horse?
   Expected reply: George Washingtons horse was white.
*/
+ what color was * (@colors) *
- <formal> <star3> was <star2>.

/* Arrays
   ------
   Human says:     I have a yellow sports car
   Expected reply: Why did you choose that color for a sports car?
*/
+ i have a @colors *
- Why did you choose that color for a <star>?

/* Priority Triggers
   -----------------
   Human says:     I have a black davenport
   Expected reply: That's a word that's not used much anymore.
   Extra notes:    This would normally match the trigger above, but this one has
                   a high priority and matches first, even though the trigger
                   above has more words and is a more specific match.
*/
+ {weight=100}* davenport
- That's a word that's not used much anymore.

/* Unique Triggers
   ---------------
   Human says:     Tell me something interesting
   Expected reply: I will only say this once!
   Human says:     Tell me something interesting
   Expected reply: ERR: No Reply Found
*/
+ {unique} tell me something interesting
- I will only say this once!

/******************************************************************************
 * Basic Reply Testing                                                        *
 ******************************************************************************/

/* Atomic Response
   ---------------
   Human says:     how are you
   Expected reply: I'm great.
*/
+ how are you
- I'm great.

/* Random Response
   ---------------
   Human says:     hello
                   hi
                   hey
   Expected reply: Hey there!
                   Hello!
                   Hi!
*/
+ (hello|hi|hey)
- Hey there!
- Hello!
- Hi!

/* Random Response
   ---------------
   Human says:     my name is Casey
   Expected reply: Nice to meet you, Casey.
                   Hi, Casey, my name is RiveScript Test Bot.
                   Casey, nice to meet you.
   Extra notes:    This would also set the var name=Casey for the user.
*/
+ my name is *
- <set name=<formal>>Nice to meet you, <formal>.
- <set name=<formal>>Hi, <formal>, my name is <bot name>.
- <set name=<formal>><formal>, nice to meet you.

/* Weighted Random Response
   ------------------------
   Human says:     Tell me a secret
   Expected reply: I won't tell you a secret.
                   You can't handle a secret.
                   Okay, here's a secret... nope, just kidding.
                   Actually, I just don't have any secrets.
*/
+ tell me a secret
- I won't tell you a secret.{weight=20}
- You can't handle a secret.{weight=20}
- Okay, here's a secret... nope, just kidding.{weight=5}
- Actually, I just don't have any secrets.

/******************************************************************************
 * Command Testing                                                            *
 ******************************************************************************/

/* %Previous
   ---------
   Human says:     Knock, knock.
   Expected reply: Who's there?
   Human says:     Banana.
   Expected reply: Banana who?
   Human says:     Knock, knock.
   Expected reply: Who's there?
   Human says:     Banana.
   Expected reply: Banana who?
   Human says:     Knock, knock.
   Expected reply: Who's there?
   Human says:     Orange.
   Expected reply: Orange who?
   Human says:     Orange you glad I didn't say banana?
   Expected reply: Haha! "Orange you glad I didn't say banana"! :D
*/
+ knock knock
- Who's there?

+ *
% who is there
- <set joke=<star>><sentence> who?

+ <get joke> *
- Haha! "{sentence}<get joke> <star>{/sentence}"! :D

/* ^Continue
   ---------
   Human says:     Tell me a poem
   Expected reply: Little Miss Muffit sat on her tuffet
                     in a nonchalant sort of way.
                     With her forefield around her,
                     the Spider, the bounder,
                     Is not in the picture today.
*/
+ tell me a poem
- Little Miss Muffit sat on her tuffet\n
^ in a nonchalant sort of way.\n
^ With her forcefield around her,\n
^ the Spider, the bounder,\n
^ Is not in the picture today.

/* @Redirect
   ---------
   Human says:     Who are you?
   Expected reply: You can call me RiveScript Test Bot.
*/
+ who are you
@ what is your name

/* @Redirect
   ---------
   Human says:     Test recursion
   Expected reply: ERR: Deep Recursion Detected!
*/
+ test recursion
@ test more recursion

+ test more recursion
@ test recursion

/* Conditionals
   ------------
   Human says:     What am I old enough to do?
   Expected reply: You never told me how old you are.
                   You're too young to do much of anything.
                   You're over 18 so you can gamble.
                   You're over 21 so you can drink.
*/
+ what am i old enough to do
* <get age> == undefined => You never told me how old you are.
* <get age> >= 21        => You're over 21 so you can drink.
* <get age> >= 18        => You're over 18 so you can gamble.
* <get age> <  18        => You're too young to do much of anything.
- This reply shouldn't happen.

/* Conditionals
   ------------
   Human says:     Am I 18 years old?
   Expected reply: I don't know how old you are.
                   You're not 18, no.
                   Yes, you are.
*/
+ am i 18 years old
* <get age> == undefined => I don't know how old you are.
* <get age> != 18        => You're not 18, no.
- Yes, you are.

/* Conditionals
   ------------
   Human says:     Count.
   Expected reply: Let's start with 1.
   Human says:     Count.
   Expected reply: I've added 1 to the count.
   Human says:     Count.
   Expected reply: I've added 5 now.
   Human says:     Count.
   Expected reply: Subtracted 2.
   Human says:     Count.
   Expected reply: Now I've doubled that.
   Human says:     Count.
   Expected reply: Subtracted 2 from that now.
   Human says:     Count.
   Expected reply: Divided that by 2.
   Human says:     Count.
   Expected reply: Subtracted 1.
   Human says:     Count.
   Expected reply: Now I've added 3.
   Human says:     Count.
   Expected reply: Added 3 again.
   Human says:     Count.
   Expected reply: We're done. Do you know what number I stopped at?
   Human says:     9
   Expected reply: You're right, I stopped at the number 9. :)
*/
+ count
* <get count> == undefined => <set count=1>Let's start with 1.
* <get count> == 0         => <set count=1>Let's start again with 1.
* <get count> == 1         => <add count=1>I've added 1 to the count.
* <get count> == 2         => <add count=5>I've added 5 now.
* <get count> == 3         => <add count=3>Now I've added 3.
* <get count> == 4         => <sub count=1>Subtracted 1.
* <get count> == 5         => <mult count=2>Now I've doubled that.
* <get count> == 6         => <add count=3>Added 3 again.
* <get count> == 7         => <sub count=2>Subtracted 2.
* <get count> == 8         => <div count=2>Divided that by 2.
* <get count> == 9         => <set count=0>We're done. Do you know what number I
  ^ \sstopped at?
* <get count> == 10        => <sub count=2>Subtracted 2 from that now.

+ (9|nine)
% * do you know what number i stopped at
- You're right, I stopped at the number 9. :)

/******************************************************************************
 * Object Macro Testing (Perl Only)                                           *
 ******************************************************************************/

/* Encoding Object
   ---------------
   Human says:     Encode something in MD5.
   Expected reply: "something" in MD5 is: 437b930db84b8079c2dd804a71936b5f
   Human says:     Encode something in Base64.
   Expected reply: "something" in Base64 is: c29tZXRoaW5n
*/

> object encode perl
	my ($rs,$method,@args) = @_;
	my $msg = join(" ",@args);

	use Digest::MD5 qw(md5_hex);
	use MIME::Base64 qw(encode_base64);

	if ($method eq "md5") {
		return md5_hex($msg);
	}
	else {
		return encode_base64($msg);
	}
< object

+ encode * in md5
- "<star>" in MD5 is: <call>encode md5 <star></call>

+ encode * in base64
- "<star>" in Base64 is: <call>encode base64 <star></call>

> object testing javascript
	var w = screen.width;
	var h = screen.height;
	var dim = w + "x" + h;
	return dim;
< object

+ test javascript
- Testing javascript... <call>testing</call>.

! global topics = hello world

+ test global
- Testing reserved global: topics=<env topics>; users=<env users>; client=<env client>; reserved=<env reserved>.

/******************************************************************************
 * Topic Testing                                                              *
 ******************************************************************************/

/*
    Temporarily ignoring abusive users
    ----------------------------------
    Human says:     insert swear word here
    Expected reply: Omg you're mean! I'm not talking to you until you apologize.
    Human says:     (anything)
    Expected reply: Not until you apologize.
                    Say you're sorry.
                    Apologize for being so mean.
    Human says:     sorry
    Expected reply: Okay, I'll forgive you.
*/

+ insert swear word here
- Omg you're mean! I'm not talking to you until you apologize.{topic=apology}

> topic apology
	+ *
	- Not until you apologize.
	- Say you're sorry.
	- Apologize for being so mean.

	+ [*] (sorry|apologize) [*]
	- Okay, I'll forgive you.{topic=random}
< topic

/*
    Topic Inheritence (simple roleplaying game)
    -------------------------------------------
    Human says:     enter the dungeon
    Expected reply: (drops you into a mini game. Skim the code below to figure
                    it out)
*/

+ enter the dungeon
- {topic=room1}You've entered the dungeon. {@look}

> topic global
	+ help{weight=100}
	- Game Help (todo)

	+ inventory{weight=100}
	- Your Inventory (todo)

	+ (north|n|south|s|east|e|west|w)
	- You can't go in that direction.

	+ quit{weight=100}
	- {topic=random}Quitter!

	+ _ *
	- You don't need to use the word "<star>" in this game.

	+ *
	- I don't understand what you're saying. Try "help" or "quit".
< topic

> topic dungeon inherits global
	+ hint
	- What do you need a hint on?\n
	^ * How to play\n
	^ * About this game

	+ how to play
	% what do you need a hint *
	- The commands are "help", "inventory", and "quit". Just read and type.

	+ about this game
	% what do you need a hint *
	- This is just a sample RPG game to demonstrate topic inheritence.
< topic

> topic room1 inherits dungeon
	+ look
	- You're in a room with a large number "1" on the floor.\s
	^ Exits are north and east.

	+ (north|n){weight=5}
	- {topic=room2}{@look}

	+ (east|e){weight=5}
	- {topic=room3}{@look}
< topic

> topic room2 inherits dungeon
	+ look
	- This room has the number "2" here. There's a flask here that's trapped
	^ \sin some kind of mechanism that only opens while the button is held
	^ \sdown (so, hold down the button then quickly grab the flask).\n\n
	^ The only exit is to the south.

	+ [push|press|hold] button [*]
	- You press down on the button and the mechanism holding the flask is\s
	^ unlocked.

	+ [take|pick up] [ye] flask [*]
	% * mechanism holding the flask is unlocked
	- You try to take ye flask but fail (you can't take ye flask, give up).

	+ [take|pick up] [ye] flask [*]
	- You can't get ye flask while the mechanism is holding onto it.

	+ (south|s){weight=5}
	- {topic=room1}{@look}
< topic

> topic room3 inherits dungeon
	+ look
	- There's nothing here but the number "3". Only exit is to the west.

	+ (west|w){weight=5}
	- {topic=room1}{@look}
< topic
