use eyre::{Context, Result};
use rcgen::{DistinguishedName, DnType, IsCa, KeyUsagePurpose};
use time::Duration;

pub fn generate_cert() -> Result<(tonic::transport::Identity, rcgen::Certificate)> {
    // https://github.com/hashicorp/go-plugin/blob/8d2aaa458971cba97c3bfec1b0380322e024b514/mtls.go#L20
    let keypair = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256)
        .wrap_err("failed to generate keypair")?;

    let mut params =
        rcgen::CertificateParams::new(["localhost".to_owned()]).wrap_err("creating cert params")?;
    params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyEncipherment,
        KeyUsagePurpose::KeyAgreement,
        KeyUsagePurpose::KeyCertSign,
    ];
    params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    params.not_before = time::OffsetDateTime::now_utc().saturating_add(Duration::seconds(-30));
    params.not_after = time::OffsetDateTime::now_utc().saturating_add(Duration::seconds(262_980));
    let mut dn = DistinguishedName::new();
    dn.push(DnType::OrganizationName, "HashiCorp");
    dn.push(DnType::CommonName, "localhost");
    params.distinguished_name = dn;

    let cert = params.self_signed(&keypair).wrap_err("signing cert")?;

    Ok((tonic::transport::Identity::from_pem(cert.pem(), keypair.serialize_pem()), cert))
}
