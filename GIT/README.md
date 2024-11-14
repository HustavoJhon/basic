# Delete Commit for Back

```git
git log
git reset --hard <commit>
git push origin <branch> --force
git commit -m "Nuevo mensaje para el commit" --date="2024-11-12T15:00:00"
git reset --soft HEAD~1
```

[On undoing, fixing, or removing commits in git](https://sethrobertson.github.io/GitFixUm/fixup.html#remove_last)
