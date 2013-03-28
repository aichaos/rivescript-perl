use strict;
use warnings;

use Test::More;
use RiveScript;

my $rs = RiveScript->new;

$rs->stream(q~
    + {unique}tell me something
    - I will only say this once!

    + {unique}what is the *
    - I don't know
~);

$rs->sortReplies;

is( $rs->reply("user1", "tell me something"), "I will only say this once!" );
is( $rs->reply("user1", "tell me something"), "ERR: No Reply Found" );
is( $rs->reply("user2", "tell me something"), "I will only say this once!" );
is( $rs->reply("user2", "tell me something"), "ERR: No Reply Found" );
is( $rs->reply("user1", "what is the time"), "I don't know" );
is( $rs->reply("user1", "what is the time"), "ERR: No Reply Found" );
is( $rs->reply("user2", "what is the time"), "I don't know" );
is( $rs->reply("user2", "what is the time"), "ERR: No Reply Found" );

done_testing;
