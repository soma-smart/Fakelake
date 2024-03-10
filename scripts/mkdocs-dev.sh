if ! mkdocs --version &> /dev/null
then
    echo "You need to install mkdocs to be able to run the docs locally"
    echo "pip install mkdocs"
    exit 1
fi

mkdocs serve