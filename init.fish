if command -sq nutshell
    nutshell init fish | source
else
    echo 'nutshell: command not found, please install it from https://github.com/mishakrpv/nutshell'
end