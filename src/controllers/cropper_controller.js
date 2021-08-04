import Cropper from 'cropperjs';
import { Controller } from 'stimulus';

export default class extends Controller {
    static targets = ["crop"];

    connect() {
        var cropBoxData;
        var canvasData;
        console.log('here');
        const image = document.getElementById('image');
        const cropper = new Cropper(image, {
            autoCropArea: 0.5,
            ready: function () {
                //Should set crop box data first here
                cropper.setCropBoxData(cropBoxData).setCanvasData(canvasData);
            }
        });
    }
}
