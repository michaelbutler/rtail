
setup() {
    load 'test_helper/bats-support/load'
    load 'test_helper/bats-assert/load'
    # ... the remaining setup is unchanged

    # get the containing directory of this file
    # use $BATS_TEST_FILENAME instead of ${BASH_SOURCE[0]} or $0,
    # as those will point to the bats executable's location or the preprocessed file respectively
    DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" >/dev/null 2>&1 && pwd )"
    # make executables in src/ visible to PATH
    PATH="$DIR/../src:$PATH"
}

@test "Test with one line output, no trailing newline" {
    run /usr/bin/rtail -n 1 /code/test/testfile.txt
    assert_output "pocket, he strode away with as rapid a motion as the wind and the rain"
}

@test "Test with one line output, no trailing newline (md5)" {
    run sh -c '/usr/bin/rtail -n 1 /code/test/testfile.txt | md5sum | cut -d" " -f1'
    # assert_output seems to ignore trailing newlines in the output so we need to
    # really assert the output is exact using md5sum
    assert_output "8b8784ea85a283d519aa481c1bd87b3e"
}

@test "Test with one line output, with trailing newline (md5)" {
    run sh -c '/usr/bin/rtail -n 1 /code/test/testfile_newline.txt | md5sum | cut -d" " -f1'
    # assert_output seems to ignore trailing newlines in the output so we need to
    # really assert the output is exact using md5sum
    assert_output "58d1705ce58536877a39a30d0a6e74e7"
}

@test "Test with two line output, no trailing newline" {
    run /usr/bin/rtail -n 2 /code/test/testfile.txt
    assert_output "thing proffered _might_ do as well; and thrusting it into his ample
pocket, he strode away with as rapid a motion as the wind and the rain"
}

@test "Test with one line output, trailing newline" {
    run /usr/bin/rtail -n 1 /code/test/testfile_newline.txt
    assert_output "pocket, he strode away with as rapid a motion as the wind and the rain"
}

@test "Test with two line output, trailing newline" {
    run /usr/bin/rtail -n 2 /code/test/testfile_newline.txt
    assert_output "thing proffered _might_ do as well; and thrusting it into his ample
pocket, he strode away with as rapid a motion as the wind and the rain"
}

@test "Test when c is greater than number of chars in file" {
    run /usr/bin/rtail -c 9999 /code/test/testfile.txt
    assert_output "It was a dark and stormy night; the rain fell in torrents, except at
occasional intervals, when it was checked by a violent gust of wind which
swept up the streets (for it is in London that our scene lies), rattling
along the house-tops, and fiercely agitating the scanty flame of the
lamps that struggled against the darkness.  Through one of the obscurest
quarters of London, and among haunts little loved by the gentlemen of the
police, a man, evidently of the lowest orders, was wending his solitary
way.  He stopped twice or thrice at different shops and houses of a
description correspondent with the appearance of the _quartier_ in which
they were situated, and tended inquiry for some article or another which
did not seem easily to be met with.  All the answers he received were
couched in the negative; and as he turned from each door he muttered to
himself, in no very elegant phraseology, his disappointment and
discontent.  At length, at one house, the landlord, a sturdy butcher,
after rendering the same reply the inquirer had hitherto received, added,
\"But if _this_ vill do as vell, Dummie, it is quite at your sarvice!\"
Pausing reflectively for a moment, Dummie responded that he thought the
thing proffered _might_ do as well; and thrusting it into his ample
pocket, he strode away with as rapid a motion as the wind and the rain"
}

@test "Test when n is greater than number of lines in file" {
    run /usr/bin/rtail -n 9999 /code/test/testfile.txt
    assert_output "It was a dark and stormy night; the rain fell in torrents, except at
occasional intervals, when it was checked by a violent gust of wind which
swept up the streets (for it is in London that our scene lies), rattling
along the house-tops, and fiercely agitating the scanty flame of the
lamps that struggled against the darkness.  Through one of the obscurest
quarters of London, and among haunts little loved by the gentlemen of the
police, a man, evidently of the lowest orders, was wending his solitary
way.  He stopped twice or thrice at different shops and houses of a
description correspondent with the appearance of the _quartier_ in which
they were situated, and tended inquiry for some article or another which
did not seem easily to be met with.  All the answers he received were
couched in the negative; and as he turned from each door he muttered to
himself, in no very elegant phraseology, his disappointment and
discontent.  At length, at one house, the landlord, a sturdy butcher,
after rendering the same reply the inquirer had hitherto received, added,
\"But if _this_ vill do as vell, Dummie, it is quite at your sarvice!\"
Pausing reflectively for a moment, Dummie responded that he thought the
thing proffered _might_ do as well; and thrusting it into his ample
pocket, he strode away with as rapid a motion as the wind and the rain"
}

@test "Test one char output, no trailing newline" {
    run /usr/bin/rtail -c 1 /code/test/testfile.txt
    assert_output "n"
}

@test "Test five char output, no trailing newline" {
    run /usr/bin/rtail -c 5 /code/test/testfile.txt
    assert_output " rain"
}

@test "Test three char output, no trailing newline (md5)" {
    run sh -c '/usr/bin/rtail -c 3 /code/test/testfile.txt | md5sum | cut -d" " -f1'
    assert_output "bf06d69212eb183731e109fecd1c89e7"
}

@test "Test three char output, trailing newline (md5)" {
    run sh -c '/usr/bin/rtail -c 3 /code/test/testfile_newline.txt | md5sum | cut -d" " -f1'
    assert_output "ba8d2b9408ed255ee92a112fe7ba59be"
}