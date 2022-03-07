# Roudmap
All desired features for this project will be documented in this file.

## Debug and Verbose
I shall map on each parameter variable with dbg_call on the start of the sensible functions.
I shall map on each variable assigment with dbg_step on the plain body of the sensible functions.
I shall map on each variable assigment with dbg_tell on the iterations of the sensible functions.
I shall map on each return expressions with dbg_reav on the conclusion of the sensible functions.
I shall map erros with dbg_erro on each returned Displayable error inside the sensible functions.
I shall map erros with dbg_bleb on each returned LizError inside the sensible functions.

## Implement WildCards on Paths
[EVAL](roud/wildcards.md) How to implement WildCards on Paths

## Independent thread to write the archive log file
[TODO] Probably is better to let the user tp pipe the stdout to some file by the OS system