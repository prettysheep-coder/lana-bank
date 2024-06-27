export function isWebAuthnSupported() {
  return (
    window.PublicKeyCredential !== undefined &&
    typeof window.PublicKeyCredential === "function"
  )
}

interface SerializedPublicKeyCredentialDescriptor
  extends Omit<PublicKeyCredentialDescriptor, "id"> {
  id: string
}

interface SerializedPublicKeyCredentialUserEntity
  extends Omit<PublicKeyCredentialUserEntity, "id"> {
  id: string
}

export interface SerializedPublicKeyCredentialCreationOptions
  extends Omit<
    PublicKeyCredentialCreationOptions,
    "challenge" | "excludeCredentials" | "user"
  > {
  challenge: string
  excludeCredentials?: SerializedPublicKeyCredentialDescriptor[]
  user: SerializedPublicKeyCredentialUserEntity
}

export interface SerializedPublicKeyCredentialRequestOptions
  extends Omit<PublicKeyCredentialRequestOptions, "challenge" | "allowCredentials"> {
  challenge: string
  allowCredentials?: SerializedPublicKeyCredentialDescriptor[]
}

interface SerializedRegisterPublicKeyCredential {
  id: string
  rawId: string
  type: string
  extensions: AuthenticationExtensionsClientOutputs
  response: {
    attestationObject: string
    clientDataJSON: string
    transports?: string[]
  }
}

function deserializePublicKeyCredentialCreationOptions(
  serializedPublicKey: SerializedPublicKeyCredentialCreationOptions,
): PublicKeyCredentialCreationOptions {
  return {
    ...serializedPublicKey,
    challenge: safeBase64UrlToArrayBuffer(serializedPublicKey.challenge),
    excludeCredentials: serializedPublicKey.excludeCredentials
      ? serializedPublicKey.excludeCredentials.map((serializedCredential) => ({
          ...serializedCredential,
          id: safeBase64UrlToArrayBuffer(serializedCredential.id),
        }))
      : undefined,
    user: {
      ...serializedPublicKey.user,
      id: safeBase64UrlToArrayBuffer(serializedPublicKey.user.id),
    },
  }
}

export function serializeRegisterCredential(
  credential: PublicKeyCredential,
): SerializedRegisterPublicKeyCredential {
  const attestationResponse = credential.response as AuthenticatorAttestationResponse

  return {
    id: credential.id,
    rawId: arrayBufferToSafeBase64Url(credential.rawId),
    type: credential.type,
    extensions: credential.getClientExtensionResults(),
    response: {
      attestationObject: arrayBufferToSafeBase64Url(
        attestationResponse.attestationObject,
      ),
      clientDataJSON: arrayBufferToSafeBase64Url(attestationResponse.clientDataJSON),
      transports:
        typeof attestationResponse.getTransports === "function"
          ? attestationResponse.getTransports()
          : undefined,
    },
  }
}

export async function signupWithPasskey(
  publicKey: SerializedPublicKeyCredentialCreationOptions,
) {
  const credentials = await navigator.credentials.create({
    publicKey: deserializePublicKeyCredentialCreationOptions(publicKey),
  })
  if (!credentials) {
    throw new Error("Browser could not create credentials.")
  }

  return JSON.stringify(serializeRegisterCredential(credentials as PublicKeyCredential))
}

export function arrayBufferToSafeBase64Url(buffer: ArrayBuffer) {
  const array = new Uint8Array(buffer)

  let string = ""
  for (let i = 0; i < array.byteLength; i++) {
    string += String.fromCharCode(array[i])
  }

  return btoa(string).replace(/\+/g, "-").replace(/\//g, "_").replace(/=*$/g, "")
}

export function safeBase64UrlToArrayBuffer(base64Url: string): ArrayBuffer {
  const base64 = atob(base64Url.replace(/-/g, "+").replace(/_/g, "/"))
  const bytes = new Uint8Array(base64.length)
  for (let i = 0; i < base64.length; i++) {
    bytes[i] = base64.charCodeAt(i)
  }

  return bytes
}
