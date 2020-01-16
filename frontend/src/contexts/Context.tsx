import React, { useContext, useState, createContext } from "react";

import { DEFAULT_USER, User, Invitation } from "models";
import { retreiveAuth } from "api/Api";

interface IAuthContextInterface {
  auth: User;
  setAuthStatus: (auth: User) => void;
  setUnauthStatus: () => void;
}

export const authContext = React.createContext<IAuthContextInterface>({
  auth: DEFAULT_USER,
  setAuthStatus: () => {},
  setUnauthStatus: () => {}
});

const { Provider } = authContext;

const AuthProvider: React.FC<{ children: React.ReactNode }> = ({
  children
}) => {
  const [auth, setAuth] = React.useState(retreiveAuth("/auth", "GET"));

  const setAuthStatus = (auth: User) => {
    retreiveAuth("/auth", "POST");
    setAuth(auth);
  };

  const setUnauthStatus = () => {
    retreiveAuth("/auth", "DELETE");
    setAuth(DEFAULT_USER);
  };

  return (
    <Provider value={{ auth, setAuthStatus, setUnauthStatus }}>
      {children}
    </Provider>
  );
};

export default AuthProvider;
