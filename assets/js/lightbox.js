function is_imagelink(url) {
    var p = /([a-z\-_0-9\/\:\.]*\.(jpg|jpeg|png|gif|webp))/i;
    return (url.match(p)) ? true : false;
}

document.addEventListener("DOMContentLoaded", function () {
    // Crée le conteneur pour le lightbox
    var lightbox = document.createElement("div");
    lightbox.setAttribute('id', "lightbox");
    lightbox.style.display = 'none';
    lightbox.style.position = 'fixed';
    lightbox.style.top = '0';
    lightbox.style.left = '0';
    lightbox.style.width = '100%';
    lightbox.style.height = '100%';
    lightbox.style.background = 'rgba(0, 0, 0, 0.8)';
    lightbox.style.zIndex = '1000';
    lightbox.style.justifyContent = 'center';
    lightbox.style.alignItems = 'center';
    lightbox.innerHTML = `<a id="close" style="position: absolute; top: 20px; right: 20px; font-size: 24px; color: white; cursor: pointer;">&times;</a>
                          <img id="lightbox-img" src="" alt="" style="max-width: 90%; max-height: 90%;"/>`;
    document.body.appendChild(lightbox);

    // Gère la fermeture du lightbox
    lightbox.addEventListener("click", function (event) {
        if (event.target.id === 'lightbox' || event.target.id === 'close') {
            lightbox.style.display = 'none';
        }
    });

    // Ajoute des classes lightbox aux balises img
    var images = document.querySelectorAll('img');
    images.forEach(image => {
        var url = image.getAttribute('src');
        if (url && is_imagelink(url)) {
            image.classList.add('lightbox-image');
        }
    });

    // Affiche l'image dans le lightbox
    var lightboxImages = document.querySelectorAll('img.lightbox-image');
    lightboxImages.forEach(image => {
        image.addEventListener("click", function () {
            var lightboxImg = document.getElementById('lightbox-img');
            lightboxImg.setAttribute('src', this.getAttribute('src'));
            lightboxImg.setAttribute('alt', this.getAttribute('alt') || '');
            lightbox.style.display = 'flex';
        });
    });
});
