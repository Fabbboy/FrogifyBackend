const firebaseConfig = {
    apiKey: "AIzaSyDekDvFeWi_YXHxu9yTxVItlpNXWpb3JzQ",
    authDomain: "frogify-18287.firebaseapp.com",
    projectId: "frogify-18287",
    storageBucket: "frogify-18287.appspot.com",
    messagingSenderId: "835340601197",
    appId: "1:835340601197:web:c09b74f2d3721178823c64",
    measurementId: "G-WH77ZLHS6H"
};
firebase.initializeApp(firebaseConfig);

const storage = firebase.storage();
const imageGrid = document.getElementById('image-grid');

storage.ref().listAll().then(function(result) {
    result.items.forEach(function(imageRef) {
        imageRef.getDownloadURL().then(function(url) {
            const card = document.createElement('div');
            card.className = 'card';

            const image = document.createElement('img');
            image.src = url;
            card.appendChild(image);

            imageGrid.appendChild(card);
        });
    });
}).catch(function(error) {
    console.log(error);
});
