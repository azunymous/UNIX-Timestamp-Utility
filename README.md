# UNIX-Timestamp-Utility
Utility for converting date/time to and from UNIX timestamps and renaming files to UNIX timestamps.
Designed for millisecond UNIX timestamps. 

Future Plans:

- Remove directories from file count
- Change returned errors to a more well defined type / prexisting type
- Combine Generate and Check commands - automatically detect whether DateTime or Unix timestamp.

## Install
Clone the repository and then run cargo install. You'll need rust & cargo. 

``git clone https://github.com/azunymous/UNIX-Timestamp-Utility.git
cd UNIX-Timestamp-Utility
cargo install``

## Use
``timestamp -h ``

``timestamp generate 2018-03-16 21:33:56.855 ``

``timestamp generate -c 2018-03-16 21:33:56.855 ``

``timestamp check 1521236036855 ``

``timestamp rename -r -u 152126036855 screencaps/ ``

Useful for changing downloaded files back to a Unix filename or seeing the date a filename was uploaded/created. 