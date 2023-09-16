# vdj\_to\_sdj

This is a rust application that does some limited conversion of metadata from VirtualDJ to Serato DJ.

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

## Warning

**It should go without saying that this is a very limited use application - in it's present form it would need to be customized for use, and could absolutely mangle your metadata or destroy your files. Use with caution and backups!**
