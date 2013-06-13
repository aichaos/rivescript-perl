#!/usr/bin/perl

# RiveScript Unit Tests
use strict;
use Test::More tests => 29;

use_ok('RiveScript');
my @tests;

# Constants.
my $MATCH = RiveScript::RS_ERR_MATCH();
my $REPLY = RiveScript::RS_ERR_REPLY();

#-----------------------------------------------------------------------------#
# Begin Block Tests                                                           #
#-----------------------------------------------------------------------------#

push @tests, sub {
    # No begin blocks.
    my $rs = bot("
        + hello bot
        - Hello human.
    ");
    test($rs, "Hello Bot", "Hello human.", "No begin block.");
    test($rs, "How are you?", $MATCH, "No trigger matched.");
};

push @tests, sub {
    # Simple begin blocks.
    my $rs = bot("
        > begin
            + request
            - {ok}
        < begin

        + hello bot
        - Hello human.
    ");
    test($rs, "Hello Bot", "Hello human.", "Simple begin block.");
};

push @tests, sub {
    # 'Blocked' begin blocks.
    my $rs = bot("
        > begin
            + request
            - Nope.
        < begin

        + hello bot
        - Hello human.
    ");
    test($rs, "Hello Bot", "Nope.", "Begin blocks access to real reply.");
};

push @tests, sub {
    # Conditional begin blocks.
    my $rs = bot("
        > begin
            + request
            * <get met> == undefined => <set met=true>{ok}
            * <get name> != undefined => <get name>: {ok}
            - {ok}
        < begin

        + hello bot
        - Hello human.

        + my name is *
        - <set name=<formal>>Hello, <get name>.
    ");
    test($rs, 'Hello bot.', 'Hello human.', 'Trigger works.');
    tv($rs, 'met', 'true', '"met" variable set to true.');
    tv($rs, 'name', undef, '"name" is still undefined.');
    test($rs, 'My name is bob', 'Hello, Bob.', 'Set user name.');
    tv($rs, 'name', 'Bob', '"name" was successfully set.');
    test($rs, 'Hello Bot', 'Bob: Hello human.', 'Name condition worked.');
};

#-----------------------------------------------------------------------------#
# Bot vars & substitutions                                                    #
#-----------------------------------------------------------------------------#

push @tests, sub {
    # Bot vars.
    my $rs = bot("
        ! var name = Aiden
        ! var age  = 5

        + what is your name
        - My name is <bot name>.

        + how old are you
        - I am <bot age>.

        + what are you
        - I'm <bot gender>.
    ");
    test($rs, 'What is your name?', 'My name is Aiden.', 'Bot name.');
    test($rs, 'How old are you?', 'I am 5.', 'Bot age.');
    test($rs, 'What are you?', "I'm undefined.", 'Undefined bot variable.');
};

push @tests, sub {
    # Before and after subs.
    my $rs = bot("
        + whats up
        - nm.

        + what is up
        - Not much.
    ");
    test($rs, 'whats up', 'nm.', 'Literal "whats up"');
    test($rs, 'what\'s up', 'nm.', 'Literal "what\'s up"');
    test($rs, 'what is up', 'Not much.', 'Literal "what is up"');

    # Add subs
    extend($rs, "
        ! sub whats  = what is
        ! sub what's = what is
    ");
    test($rs, 'whats up', 'Not much.', 'Literal "whats up"');
    test($rs, 'what\'s up', 'Not much.', 'Literal "what\'s up"');
    test($rs, 'what is up', 'Not much.', 'Literal "what is up"');
};

push @tests, sub {
    # Before and after person subs.
    my $rs = bot("
        + say *
        - <person>
    ");
    test($rs, 'say i am cool', 'i am cool', 'Person substitution 1');
    test($rs, 'say you are dumb', 'you are dumb', 'Person substitution 2');

    extend($rs, "
        ! person i am    = you are
        ! person you are = I am
    ");
    test($rs, 'say i am cool', 'you are cool', 'Person substitution 3');
    test($rs, 'say you are dumb', 'I am dumb', 'Person substitution 4');
};

#-----------------------------------------------------------------------------#
# Triggers                                                                    #
#-----------------------------------------------------------------------------#

push @tests, sub {
    # Atomic & Wildcard
    my $rs = bot("
        + hello bot
        - Hello human.

        + my name is *
        - Nice to meet you, <star>.

        + * told me to say *
        - Why did <star1> tell you to say <star2>?

        + i am # years old
        - A lot of people are <star>.

        + i am _ years old
        - Say that with numbers.

        + i am * years old
        - Say that with fewer words.
    ");
    test($rs, 'hello bot', 'Hello human.', 'Atomic trigger.');
    test($rs, 'my name is Bob', 'Nice to meet you, bob.', 'One star.');
    test($rs, 'bob told me to say hi', 'Why did bob tell you to say hi?',
        'Two stars.');
#    test($rs, 'i am 5 years old', 'A lot of people are 5.', 'Number wildcard.');
    test($rs, 'i am five years old', 'Say that with numbers.',
        'Underscore wildcard.');
    test($rs, 'i am twenty five years old', 'Say that with fewer words.',
        'Star wildcard.');
};

#-----------------------------------------------------------------------------#
# End Unit Tests                                                              #
#-----------------------------------------------------------------------------#

# Run all the tests.
for my $t (@tests) {
    $t->();
}

### Utility Functions ###

# Make a new bot
sub bot {
    my $code = shift;
    my $rs = RiveScript->new();
    return extend($rs, $code);
}

# Extend a bot.
sub extend {
    my ($rs, $code) = @_;
    $rs->stream($code);
    $rs->sortReplies();
    return $rs;
}

# Test message and response.
sub test {
    my ($rs, $in, $out, $note) = @_;
    my $reply = $rs->reply('user', $in);
    is($reply, $out, $note);
}

# Test user variable.
sub tv {
    my ($rs, $var, $value, $note) = @_;
    is($rs->getUservar('user', $var), $value, $note);
}
