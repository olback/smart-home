import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { DashComponent } from './dash/dash.component';
import { TemperatureComponent } from './temperature/temperature.component';
import { WeatherComponent } from './weather/weather.component';
import { SpeakersComponent } from './speakers/speakers.component';
import { TradfriComponent } from './tradfri/tradfri.component';
import { ShoppingListComponent } from './shopping-list/shopping-list.component';
import { LocationsComponent } from './locations/locations.component';
import { AboutComponent } from './about/about.component';
import { NotificationsComponent } from './notifications/notifications.component';

const routes: Routes = [
  {
    path: '',
    pathMatch: 'full',
    redirectTo: '/dash'
  },
  {
    path: 'dash',
    component: DashComponent
  },
  {
    path: 'temperature',
    component: TemperatureComponent
  },
  {
    path: 'weather',
    component: WeatherComponent
  },
  {
    path: 'speakers',
    component: SpeakersComponent
  },
  {
    path: 'tradfri',
    component: TradfriComponent
  },
  {
    path: 'shopping-list',
    component: ShoppingListComponent
  },
  {
    path: 'notifications',
    component: NotificationsComponent
  },
  {
    path: 'locations',
    component: LocationsComponent
  },
  {
    path: 'about',
    component: AboutComponent
  },
  {
    path: '**',
    pathMatch: 'full',
    redirectTo: '/dash'
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
