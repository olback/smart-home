import { NgModule } from '@angular/core';
import { APP_BASE_HREF } from '@angular/common';
import { BrowserModule } from '@angular/platform-browser';
import { AppRoutingModule } from './app-routing.module';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { ChartsModule } from 'ng2-charts';

// Angular Material
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatListModule } from '@angular/material/list';
import { MatCardModule } from '@angular/material/card';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatSliderModule } from '@angular/material/slider';
import { MatBadgeModule } from '@angular/material/badge';

// Components
import { AppComponent } from './app.component';
import { WeatherComponent } from './weather/weather.component';
import { TemperatureComponent } from './temperature/temperature.component';
import { SpeakersComponent } from './speakers/speakers.component';
import { TradfriComponent } from './tradfri/tradfri.component';
import { ShoppingListComponent } from './shopping-list/shopping-list.component';
import { LocationsComponent } from './locations/locations.component';
import { AboutComponent } from './about/about.component';
import { DashComponent } from './dash/dash.component';
import { NotificationsComponent } from './notifications/notifications.component';

@NgModule({
  declarations: [
    AppComponent,
    WeatherComponent,
    TemperatureComponent,
    SpeakersComponent,
    TradfriComponent,
    ShoppingListComponent,
    LocationsComponent,
    AboutComponent,
    DashComponent,
    NotificationsComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    ChartsModule,
    // Angular Material
    MatToolbarModule,
    MatIconModule,
    MatButtonModule,
    MatSidenavModule,
    MatListModule,
    MatCardModule,
    MatProgressBarModule,
    MatSliderModule,
    MatBadgeModule
  ],
  providers: [{ provide: APP_BASE_HREF, useValue: '/app' }],
  bootstrap: [AppComponent]
})
export class AppModule { }
