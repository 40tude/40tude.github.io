---
layout: default
title: "SQLite et CVI"
parent: "ANSI C"
#math: mathjax
date: 2012-05-18 16:45:04
last_modified_date: 2020-05-03 18:12:19
---

# SQLite et CVI

Un rapide tutoriel qui montre comment utiliser SQLite et CVI. Il faut commencer par récupérer sur le site de SQLite (<http://www.sqlite.org/download.html>) les trois fichiers zip qui vont bien :

1. sqlite-amalgamation-xxx.zip
2. sqlite-dll-win32-x86-xxx.zip
3. sqlite-shell-win32-x86-xxx.zip

Dans le répertoire Téléchargements de Windows dézippez ensuite les fichiers en question.

Dans le répertoire "amalgamation" copiez sqlite3.h et collez le dans un répertoire nommé `testsql` par exemple.

Dans le répertoire "sqlite-dll" copiez le fichier sqlite3.dll et collez-le dans le répertoire  `testsql` précédent.

Dans le répertoire "shell" copiez le fichier sqlite3.exe et collez-le dans le répertoire  `testsql` précédent.

Toujours dans le gestionnaire de fichier Windows, faites **SHIFT + Click Droit** sur le nom du répertoire  `testsql` et choisissez l'option "Ouvrir une fenêtre de commande ici". On va tester SQLite afin de voir que tout va bien. Dans la console tapez simplement :

```powershell
sqlite3 Mabase.db
create table MaTable(Nom varchar(20), Prenom varchar(20));
insert into MaTable values("BAUCOUR", "Philippe");
select * from Matable
.exit
```

N'oubliez pas le point-virgule en fin d'instruction. Si tout s'est bien passé, à la suite de l'instruction "select" vous avez récupéré une sortie écran qui correspond à ce que vous aviez rentré. Bref on a une base est SQLite fonctionne.

Ouvrez maintenant CVI est créez un nouveau projet vide

Depuis le gestionnaire de fichiers Windows, glissez déposez le fichier sqlite3.h dans la zone projet

Double cliquez sur le fichier pour le voir apparaître dans l'éditeur de code de CVI

Sélectionnez Options/Generate DLL Import Library...

Ne sélectionnez aucune option dans la petite boite de dialogue qui apparaît

Dans la boite de sélection de fichiers qui apparaît choisissez sqlite3.dll (la dll est dans le répertoire  `testsql` que vous avez créé précédemment)

Ne prêtez pas trop attention à la fenêtre de Warning qui arrive à l'écran

Dans le répertoire  `testsql` vous devez maintenant avoir un fichier sqlite3.lib

Depuis le gestionnaire de fichiers Windows, faites un glissez-déposez de sqlite3.lib dans la fenêtre projet

Maintenant, copiez-collez le code source ci-dessous dans un nouveau code source. Il faudra lui aussi l'ajouter au projet en cours (via l'option File/Add xyz to project)

```c
#include <stdio.h>
#include "sqlite3.h"

//-----------------------------------------------------------------------------
static int callback(void *NotUsed, int argc, char **argv, char **azColName){

    for(int i=0; i<argc; i++){
        printf("%s = %s\n", azColName[i], argv[i] ? argv[i] : "NULL");
    }
    printf("\n");
    return 0;
}

//-----------------------------------------------------------------------------
int main(int argc, char **argv){

    sqlite3     *db=0;
    char        *zErrMsg=0;
    int         rc;
    const char  *SQLStatements[6];

    // Create a new AnotherTable in database
    SQLStatements[0] = "create table AnotherTable (FirstName varchar(30), LastName varchar(30), Age smallint)";

    // Insert item into AnotherTable
    SQLStatements[1] = "insert into AnotherTable values ('AA', 'BB', 10)";

    // Insert item into AnotherTable
    SQLStatements[2] = "insert into AnotherTable values ('CC', 'DD', 20)";

    // Select all data
    SQLStatements[3] = "select * from AnotherTable";

    // Remove all data
    SQLStatements[4] = "delete from AnotherTable";

    // Drop the table from database
    SQLStatements[5] = "drop table AnotherTable";

    //-----------------------------------------------------------------------------
    rc = sqlite3_open("MaBase.db", &db);
    if(rc!=SQLITE_OK){
        fprintf(stderr, "Can't open database: %s\n", sqlite3_errmsg(db));
    }else{
    rc = sqlite3_exec(db, "select * from MaTable", callback, 0, 0);
    sqlite3_close(db);
    db=0;
  }

  //-----------------------------------------------------------------------------
    rc = sqlite3_open("test.db", &db);
    if(rc!=SQLITE_OK){
        fprintf(stderr, "Can't open database: %s\n", sqlite3_errmsg(db));
    }else{
    for(int i = 0; i < 6; i++){
        rc = sqlite3_exec(db, SQLStatements[i], callback, 0, &zErrMsg);
        if( rc!=SQLITE_OK ){
            fprintf(stderr, "SQL error: %s\n", zErrMsg);
            sqlite3_free(zErrMsg);
            break;
        }
    }
    sqlite3_close(db);
    db=0;
  }

  printf("Strike ENTER to exit :");
  getchar();
  return 0;
}
```

Finalement, vous avez 3 fichiers dans la fenêtre projet : ``sqlite3.h``, ``sqlite3.lib`` et ``test.c`` (si c'est comme ça que vous avez nommé le code source ci-dessous). Allez zou, **SHIFT + F5**. Ça doit tourner. Voilà la sortie écran que j'obtiens :

```bash
Nom = BAUCOUR
Prenom = Philippe
FirstName = AA
LastName = BB
Age = 10
FirstName = CC
LastName = DD
Age = 20
Strike ENTER to exit :
```

### Explications

À la ligne 40 on ouvre la base que l'on a créé dans la console et on affiche le contenu de la table MaTable à l'écran. Notez que l'affichage se fait via une fonction callback passée en paramètre à la ligne 44. Notez aussi qu'on ne gère pas trop les erreurs.

Ligne 50, on créer et/ou ouvre une nouvelle base : test.db. Ensuite on exécute une série d'instructions SQL et on ferme la base. Le truc à noter dans cette section de code c'est sans doute la gestion des erreurs et l'usage de sqlite3_free.

Bon vous devriez être capable maintenant d'utiliser SQLite avec CVI. Trouvez-vous un bon tutoriel SQL et amusez-vous bien.

