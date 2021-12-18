# Description

This library allows to easily read any number from stdin.  

## Functions

1. read_num - returns Result<Number, Error>  
2. read_num_with_delimiters - same as read_num, except it also accepts list of delimiters 
3. read_and_handle - infinite loop that breaks on successfull number parse
4. read_and_handle_with_delimiters - same as read_and_handle, except it also accepts list of delimiters
