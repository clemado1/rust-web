import React, { useContext, useState, createContext } from "react";

import { DEFAULT_USER, User, Invitation } from "models";
import { authHandler } from "api/Api";

interface IAuthContextInterface {
  auth: User;
  setAuthStatus: (auth: Invitation) => void;
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
  const [auth, setAuth] = React.useState(authHandler("/auth", "GET"));

  const setAuthStatus = (auth: User) => {
    authHandler("/auth", "POST");
    setAuth(auth);
  };

  const setUnauthStatus = () => {
    authHandler("/auth", "DELETE");
    setAuth(DEFAULT_USER);
  };

  return (
    <Provider value={{ auth, setAuthStatus, setUnauthStatus }}>
      {children}
    </Provider>
  );
};

export default AuthProvider;
