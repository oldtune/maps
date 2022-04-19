import GoogleMapReact from 'google-map-react';


function Map(props: any) {
    return (
        <div style={{ height: '100vh', width: '100%' }}>
            <GoogleMapReact
                bootstrapURLKeys={{ key: "AIzaSyAPrzJhpA0V2ZtPpwUuwOZCaVDQn2Y1rLs" }}
                defaultCenter={props.defaultCenter}
                defaultZoom={props.zoom}
            ></GoogleMapReact>
        </div>
    )
}

export default Map