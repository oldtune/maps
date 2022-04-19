import React from 'react';
import logo from './logo.svg';
import './App.css';
import Map from "./components/Map";

function App() {
  return (
    <Map defaultCenter={{
      lat: 10.99835602,
      lng: 77.01502627
    }} zoom={1}></Map>
  );
}

export default App;
