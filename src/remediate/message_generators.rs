pub(crate) fn generate_desc_ec2_infected(instance_id: String, volume_id: String) -> String {
    format!("Trojan.Ransom.AIG was detected in Volume/s ({volume_id}) attached to the instance ({instance_id}). Gen:Heur.Ransom.MSIL.!diop!.1 was detected in Volume/s ({volume_id}) attached to the instance ({instance_id}). The Brotherhood was detected in Volume/s ({volume_id}) attached to the instance ({instance_id}).")
}

// pub(crate) fn generate_remediation()
