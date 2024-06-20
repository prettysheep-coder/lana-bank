import { getLoginFlow } from "./api/get-login-flow"
import { getRegistrationFlow } from "./api/get-registration-flow"
import { getSession } from "./api/get-session"
import { startSignInFlow } from "./api/start-login-flow"
import { startRegisterFlow } from "./api/start-register-flow"
import { verifyEmailCodeLoginFlow } from "./api/verify-login-code-flow"
import { verifyEmailCodeRegisterFlow } from "./api/verify-register-code-flow"

export const authService = () => {
  return {
    startRegisterFlow,
    startSignInFlow,
    verifyEmailCodeLoginFlow,
    verifyEmailCodeRegisterFlow,
    getLoginFlow,
    getRegistrationFlow,
    getSession,
  }
}
