import * as React from "react";

import RootContainer from "pages/Root";

/** Context API */
import AuthContextProvider from "contexts/Context";

function App() {
  return (
    <AuthContextProvider>
      <RootContainer />
    </AuthContextProvider>
  );
}

export default App;
