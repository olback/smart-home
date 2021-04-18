import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-about',
  templateUrl: './about.component.html',
  styleUrls: ['./about.component.scss']
})
export class AboutComponent implements OnInit {

  version = {
    angular: '<unavailable>',
    hash: '<unavailable>',
    client: '<unavailable>',
    server: '<unavailable>',
    api: '<unavailable>'
  };

  constructor() { }

  ngOnInit(): void {
    this.getAngularVersion();
  }

  getAngularVersion() {
    this.version.angular = document.getElementsByTagName('app-root')[0].attributes.getNamedItem('ng-version').value;
  }

}
