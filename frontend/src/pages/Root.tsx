import * as React from "react";

import { authContext } from "contexts/Context";
import Home from "pages/Home";

import Login from "./Login";

function RootContainer() {
  const { auth } = React.useContext(authContext);
  return (
    <div>
      {auth.email ? <Home /> : null}
      {!auth.email && <Login />}
    </div>
  );
}

export default RootContainer;
