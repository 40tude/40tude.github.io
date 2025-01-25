function is_imagelink(url) {
    var p = /([a-z\-_0-9\/\:\.]*\.(jpg|jpeg|png|gif|webp))/i;
    return (url.match(p)) ? true : false;
}

function setGallery(el) {
    var elements = document.body.querySelectorAll(".gallery");
    elements.forEach(element => {
        element.classList.remove('gallery');
    });

    // Récupère toutes les images de la galerie
    var img_elements = el.closest('ul, p').querySelectorAll("img[class*='lightbox-']");
    img_elements.forEach(img_element => {
        img_element.classList.remove('current');
    });

    img_elements.forEach(img_element => {
        if (el.getAttribute('src') == img_element.getAttribute('src')) {
            img_element.classList.add('current');
        }
    });

    if (img_elements.length > 1) {
        document.getElementById('lightbox').classList.add('gallery');
        img_elements.forEach(img_element => {
            img_element.classList.add('gallery');
        });
    }

    //add the next and previous buttons
    var currentkey;
    var gallery_elements = document.querySelectorAll('img.gallery');
    Object.keys(gallery_elements).forEach(function (k) {
        if (gallery_elements[k].classList.contains('current')) currentkey = k;
    });

    if (currentkey == (gallery_elements.length - 1)) var nextkey = 0;
    else var nextkey = parseInt(currentkey) + 1;

    if (currentkey == 0) var prevkey = parseInt(gallery_elements.length - 1);
    else var prevkey = parseInt(currentkey) - 1;

    document.getElementById('next').addEventListener("click", function () {
        gallery_elements[nextkey].click();
    });

    document.getElementById('prev').addEventListener("click", function () {
        gallery_elements[prevkey].click();
    });
}

document.addEventListener("DOMContentLoaded", function () {

    // Crée le conteneur pour le lightbox
    var newdiv = document.createElement("div");
    newdiv.setAttribute('id', "lightbox");
    document.body.appendChild(newdiv);

    // Ajoute des classes lightbox aux balises img
    var elements = document.querySelectorAll('img');
    elements.forEach(element => {
        var url = element.getAttribute('src');
        if (url) {
            if (is_imagelink(url) && !element.classList.contains('no-lightbox')) {
                element.classList.add('lightbox-image');
                var filename = url.split('/').pop();
                var split = filename.split(".");
                var name = split[0];
                element.setAttribute('title', name);
            }
        }
    });

    // Supprime le lightbox au clic
    document.getElementById('lightbox').addEventListener("click", function (event) {
        if (event.target.id != 'next' && event.target.id != 'prev') {
            this.innerHTML = '';
            document.getElementById('lightbox').style.display = 'none';
        }
    });

    // Ajoute l'événement pour ouvrir le lightbox sur les images
    var elements = document.querySelectorAll('img.lightbox-image');
    elements.forEach(element => {
        element.addEventListener("click", function (event) {
            event.preventDefault();
            document.getElementById('lightbox').innerHTML = `
                <a id="close"></a>
                <a id="next">&rsaquo;</a>
                <a id="prev">&lsaquo;</a>
                <div class="img" style="background: url('${this.getAttribute('src')}') center center / contain no-repeat;" title="${this.getAttribute('title')}">
                    <img src="${this.getAttribute('src')}" alt="${this.getAttribute('title')}" />
                </div>
                <span>${this.getAttribute('title')}</span>`;
            document.getElementById('lightbox').style.display = 'block';

            setGallery(this);
        });
    });
});
