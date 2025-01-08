---
layout: default
title: "Parcours récursifs de répertoires en C ANSI"
parent: "ANSI C"
#math: mathjax
date: 2013-12-26 13:46:18
last_modified_date: 2020-05-03 18:19:46
---

# Parcours récursifs de répertoires en C ANSI

Dans un récent projet j'ai dû coder un parcours de répertoires récursif. Bien sûr il fallait traiter les sous-répertoires et les fichiers afin d'appliquer un traitement particulier aux fichiers ``.c`` que je trouvais. En fait ce n'est pas possible avec les fonctions ``GetFirstFile()`` et ``GetNextFile()`` de [CVI](https://www.ni.com/cvi) car elles ne supportent pas bien la récursivité.

Voilà ce que j'ai dû écrire (attention il ne faut pas oublier d'inclure le fichier ``windows.h`` en haut du code source). N'hésitez pas à faire du copier-coller si besoin.

```c
// ----------------------------------------------------------------------------
static void WalkDirectoryTree(const char RootDir[]) {

  char DirPattern[MAX_PATHNAME_LEN];
  sprintf(DirPattern,"%s\\*", RootDir);

  WIN32_FIND_DATA ffd;
  HANDLE hFind = FindFirstFile(DirPattern, &ffd);
  if (hFind!=INVALID_HANDLE_VALUE) {
    do{
      if(ffd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY){
        if (ffd.cFileName[0] == '.') continue;
        char NewRootDir[MAX_PATHNAME_LEN];
        sprintf(NewRootDir,"%s\\%s", RootDir, ffd.cFileName);
        WalkDirectoryTree(NewRootDir);
      }
    }while(FindNextFile(hFind, &ffd));
    FindClose(hFind);
  }

  char FilePattern[MAX_PATHNAME_LEN];
  sprintf(FilePattern,"%s\\*.c", RootDir);

  hFind = FindFirstFile(FilePattern, &ffd);
  if (hFind!=INVALID_HANDLE_VALUE) {
    do{
      printf("%s\n", ffd.cFileName);
    } while (FindNextFile(hFind, &ffd));
  }
  FindClose(hFind);
}
```

En deux mots... On commence par trouver, s'il y en a, les sous-répertoires du répertoire ``RootDir`` (et on évite bien sûr de traiter les répertoires spéciaux nommés "." et ".."). Pour chaque sous-répertoire trouvé on invoque la fonction ``WalkDirectoryTree()``. Quand tous les sous répertoires ont été traités, on passe alors au traitement des fichiers contenus dans le répertoire ``RootDir``. Notez bien l'usage de ``FindClose()`` et non de la fonction ``CloseHandle()``.

Sinon en CVI voilà ce que j'avais écrit au départ (là, il ne faut pas oublier d'inclure le fichier ``toolbox.h``). Dans un premier temps on traite les fichiers que l'on trouve dans le répertoire ``RootDir``. Ensuite, s'il y a des sous répertoires, on créer une liste chaînée et on y stocke tous les noms des sous répertoires. Quand c'est fait, on parcourt la liste et on invoque de nouveau la fonction ``WalkDirectoryTree()`` pour chacun des noms de sous-répertoire rencontré. Relisez l'aide en ligne à propos de la fonction ``ListGetDataPtr()`` si besoin et notez que je créé un ``typedef char t_Dir[]`` au début de la fonction.

```c
// ----------------------------------------------------------------------------
static void WalkDirectoryTree(const char RootDir[]) {

  typedef char t_Dir[MAX_PATHNAME_LEN] ;

  char FilePattern[MAX_PATHNAME_LEN];
  sprintf(FilePattern,"%s\\*.c", RootDir);

  char FileName[MAX_FILENAME_LEN];
  if (!GetFirstFile (FilePattern, 1, 0, 0, 0, 0, 0, FileName)) {
    do{
      printf("%s\n", FileName);
    } while (!GetNextFile (FileName));
  }

  char SearchPath[MAX_PATHNAME_LEN];
  sprintf(SearchPath,"%s\\*", RootDir);

  char DirName[MAX_FILENAME_LEN];
  if (!GetFirstFile (SearchPath, 0, 0, 0, 0, 0, 1, DirName)) {
    ListType hList = ListCreate (MAX_PATHNAME_LEN);
    do{
      ListInsertItem (hList, DirName, END_OF_LIST);
    }while (!GetNextFile (DirName));

    int   numDir      = ListNumItems (hList);
    t_Dir *pListData  = (t_Dir*)ListGetDataPtr (hList);

    for (int i=0; i!=numDir; ++i) {
      sprintf(SearchPath,"%s\\%s", RootDir, *pListData);
      WalkDirectoryTree(SearchPath);
      pListData++;
    }
    ListDispose (hList);
  }
}
```

C'est un poil plus long et un poil plus lent qu'avec le SDK Windows. Le gros truc, quand même, c'est que ce n'est pas récursif et que c'est ce que je voulais au départ. Allez, comme je suis de bonne humeur et que c'était Noël il n'y a pas si longtemps, je vous donne un lien sur [GitHub](https://gist.github.com/40tude/8334627)ainsi que le code source complet qui fonctionne :


