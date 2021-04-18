import { Component, OnInit } from '@angular/core';

interface WeatherDay {
  title: string;
  high: number;
  low: number;
  icon: string;
  hourly: {
    temperature: number;
    icon: string;
    time: string;
  }[];
};

@Component({
  selector: 'app-weather',
  templateUrl: './weather.component.html',
  styleUrls: ['./weather.component.scss']
})
export class WeatherComponent implements OnInit {

  days: WeatherDay[] = [
    {
      title: 'Today',
      high: 17,
      low: 8,
      icon: 'wb_sunny',
      hourly: [
        {
          temperature: 10,
          icon: 'wb_sunny',
          time: '06:00'
        },
        {
          temperature: 11,
          icon: 'wb_sunny',
          time: '07:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '09:00'
        },
        {
          temperature: 13,
          icon: 'wb_sunny',
          time: '10:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        },
        {
          temperature: 10,
          icon: 'wb_sunny',
          time: '06:00'
        },
        {
          temperature: 11,
          icon: 'wb_sunny',
          time: '07:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '09:00'
        },
        {
          temperature: 13,
          icon: 'wb_sunny',
          time: '10:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        }
      ]
    },
    {
      title: 'Tomorrow',
      high: 17,
      low: 8,
      icon: 'wb_sunny',
      hourly: [
        {
          temperature: 10,
          icon: 'wb_sunny',
          time: '06:00'
        },
        {
          temperature: 11,
          icon: 'wb_sunny',
          time: '07:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        }
      ]
    }
    ,
    {
      title: 'Tuesday',
      high: 17,
      low: 8,
      icon: 'wb_sunny',
      hourly: [
        {
          temperature: 10,
          icon: 'wb_sunny',
          time: '06:00'
        },
        {
          temperature: 11,
          icon: 'wb_sunny',
          time: '07:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '09:00'
        },
        {
          temperature: 13,
          icon: 'wb_sunny',
          time: '10:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        }
      ]
    }
    ,
    {
      title: 'Wednesday',
      high: 17,
      low: 8,
      icon: 'wb_sunny',
      hourly: [
        {
          temperature: 10,
          icon: 'wb_sunny',
          time: '06:00'
        },
        {
          temperature: 11,
          icon: 'wb_sunny',
          time: '07:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '09:00'
        },
        {
          temperature: 13,
          icon: 'wb_sunny',
          time: '10:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        }
      ]
    },
    {
      title: 'Thursday',
      high: 17,
      low: 8,
      icon: 'wb_sunny',
      hourly: [
        {
          temperature: 10,
          icon: 'wb_sunny',
          time: '06:00'
        },
        {
          temperature: 11,
          icon: 'wb_sunny',
          time: '07:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '09:00'
        },
        {
          temperature: 13,
          icon: 'wb_sunny',
          time: '10:00'
        },
        {
          temperature: 12,
          icon: 'wb_sunny',
          time: '08:00'
        }
      ]
    }
  ]

  constructor() { }

  ngOnInit(): void {
  }

}
