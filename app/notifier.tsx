"use client"

import AWN from 'awesome-notifications';
import 'awesome-notifications/dist/style.css';

const globalOptions = {
    position: 'top-right',
    durations: {
        global: 3500,
    },
    labels: {
        tip: 'Tip',
        info: 'Info',
        success: 'Success',
        warning: 'Warning',
        alert: 'Error',
        async: 'Loading',
    },
    icons: {
        enabled: false,
    }
}

export const notifier = new AWN(globalOptions)
