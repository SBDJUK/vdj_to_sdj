# VDJ to SDJ

This is a rust application that does one-way conversion of metadata from VirtualDJ to Serato DJ.

Whilst most fields are standard tag formats, some are not, and so moving between the two applications required some comversion. There are existing applications like Lexicon and ATGR DJCU that can do this for you. Both applications have some limitations, and since I wanted a custom approach I set about creating my own tool to do the conversion exactly how I wanted.

The current version of this application performs a limited conversion of 3 pieces of information:

*   VirtualDJ Play Count → Serato DJ Playcount
*   VirtualDJ Rating → Composer
*   VirtualDJ User1 → Grouping

## Usage

Command line parameters:

--database <database.xml>

--simulate

## Current TODO list:

*   Add OGG support
*   Handle FLAC and MP4 tags better by creating a tag if the file exists but has no tag
*   Custom field mapping and combination
*   Cue points and saved loops

## Warning

**It should go without saying that this is a very limited use application - in it's present form it would need to be customized for use, and could absolutely mangle your metadata or destroy your files. Use with caution and backups!**
