launched_instances: [
              Instance {
                  ami_launch_index: Some(
                      0,
                  ),
                  architecture: Some(
                      "x86_64",
                  ),
                  block_device_mappings: Some(
                      [
                          InstanceBlockDeviceMapping {
                              device_name: Some(
                                  "/dev/sda1",
                              ),
                              ebs: Some(
                                  EbsInstanceBlockDevice {
                                      attach_time: Some(
                                          "2023-08-01T11:19:49.000Z",
                                      ),
                                      delete_on_termination: Some(
                                          true,
                                      ),
                                      status: Some(
                                          "attached",
                                      ),
                                      volume_id: Some(
                                          "vol-015c850dda54467ae",
                                      ),
                                  },
                              ),
                          },
                      ],
                  ),
                  capacity_reservation_id: None,
                  capacity_reservation_specification: Some(
                      CapacityReservationSpecificationResponse {
                          capacity_reservation_preference: Some(
                              "open",
                          ),
                          capacity_reservation_target: None,
                      },
                  ),
                  client_token: Some(
                      "",
                  ),
                  cpu_options: Some(
                      CpuOptions {
                          core_count: Some(
                              1,
                          ),
                          threads_per_core: Some(
                              2,
                          ),
                      },
                  ),
                  ebs_optimized: Some(
                      false,
                  ),
                  elastic_gpu_associations: None,
                  elastic_inference_accelerator_associations: None,
                  ena_support: Some(
                      true,
                  ),
                  enclave_options: Some(
                      EnclaveOptions {
                          enabled: Some(
                              false,
                          ),
                      },
                  ),
                  hibernation_options: Some(
                      HibernationOptions {
                          configured: Some(
                              false,
                          ),
                      },
                  ),
                  hypervisor: Some(
                      "xen",
                  ),
                  iam_instance_profile: Some(
                      IamInstanceProfile {
                          arn: Some(
                              "arn:aws:iam::233365860087:instance-profile/E2E_agentless_ssm",
                          ),
                          id: Some(
                              "AIPATMVNQN33Y2M627YFT",
                          ),
                      },
                  ),
                  image_id: Some(
                      "ami-0a53647d2a7c6c7f1",
                  ),
                  instance_id: Some(
                      "i-006ac8c980091eee6",
                  ),
                  instance_lifecycle: None,
                  instance_type: Some(
                      "m5a.large",
                  ),
                  kernel_id: None,
                  key_name: None,
                  launch_time: Some(
                      "2023-08-01T11:19:48.000Z",
                  ),
                  licenses: None,
                  metadata_options: Some(
                      InstanceMetadataOptionsResponse {
                          http_endpoint: Some(
                              "enabled",
                          ),
                          http_put_response_hop_limit: Some(
                              1,
                          ),
                          http_tokens: Some(
                              "optional",
                          ),
                          state: Some(
                              "applied",
                          ),
                      },
                  ),
                  monitoring: Some(
                      Monitoring {
                          state: Some(
                              "disabled",
                          ),
                      },
                  ),
                  network_interfaces: Some(
                      [
                          InstanceNetworkInterface {
                              association: Some(
                                  InstanceNetworkInterfaceAssociation {
                                      carrier_ip: None,
                                      ip_owner_id: Some(
                                          "amazon",
                                      ),
                                      public_dns_name: Some(
                                          "ec2-3-21-106-28.us-east-2.compute.amazonaws.com",
                                      ),
                                      public_ip: Some(
                                          "3.21.106.28",
                                      ),
                                  },
                              ),
                              attachment: Some(
                                  InstanceNetworkInterfaceAttachment {
                                      attach_time: Some(
                                          "2023-08-01T11:19:48.000Z",
                                      ),
                                      attachment_id: Some(
                                          "eni-attach-0f4a6c5050d0b400a",
                                      ),
                                      delete_on_termination: Some(
                                          true,
                                      ),
                                      device_index: Some(
                                          0,
                                      ),
                                      network_card_index: Some(
                                          0,
                                      ),
                                      status: Some(
                                          "attached",
                                      ),
                                  },
                              ),
                              description: Some(
                                  "",
                              ),
                              groups: Some(
                                  [
                                      GroupIdentifier {
                                          group_id: Some(
                                              "sg-06a99908748b8ea90",
                                          ),
                                          group_name: Some(
                                              "default",
                                          ),
                                      },
                                  ],
                              ),
                              interface_type: Some(
                                  "interface",
                              ),
                              ipv_6_addresses: Some(
                                  [],
                              ),
                              mac_address: Some(
                                  "02:04:e3:e0:3c:91",
                              ),
                              network_interface_id: Some(
                                  "eni-02573ee8d1fe19d82",
                              ),
                              owner_id: Some(
                                  "233365860087",
                              ),
                              private_dns_name: Some(
                                  "ip-172-31-7-210.us-east-2.compute.internal",
                              ),
                              private_ip_address: Some(
                                  "172.31.7.210",
                              ),
                              private_ip_addresses: Some(
                                  [
                                      InstancePrivateIpAddress {
                                          association: Some(
                                              InstanceNetworkInterfaceAssociation {
                                                  carrier_ip: None,
                                                  ip_owner_id: Some(
                                                      "amazon",
                                                  ),
                                                  public_dns_name: Some(
                                                      "ec2-3-21-106-28.us-east-2.compute.amazonaws.com",
                                                  ),
                                                  public_ip: Some(
                                                      "3.21.106.28",
                                                  ),
                                              },
                                          ),
                                          primary: Some(
                                              true,
                                          ),
                                          private_dns_name: Some(
                                              "ip-172-31-7-210.us-east-2.compute.internal",
                                          ),
                                          private_ip_address: Some(
                                              "172.31.7.210",
                                          ),
                                      },
                                  ],
                              ),
                              source_dest_check: Some(
                                  true,
                              ),
                              status: Some(
                                  "in-use",
                              ),
                              subnet_id: Some(
                                  "subnet-0e6364007229332fe",
                              ),
                              vpc_id: Some(
                                  "vpc-0872734b71255b56c",
                              ),
                          },
                      ],
                  ),
                  outpost_arn: None,
                  placement: Some(
                      Placement {
                          affinity: None,
                          availability_zone: Some(
                              "us-east-2a",
                          ),
                          group_name: Some(
                              "",
                          ),
                          host_id: None,
                          host_resource_group_arn: None,
                          partition_number: None,
                          spread_domain: None,
                          tenancy: Some(
                              "default",
                          ),
                      },
                  ),
                  platform: None,
                  private_dns_name: Some(
                      "ip-172-31-7-210.us-east-2.compute.internal",
                  ),
                  private_ip_address: Some(
                      "172.31.7.210",
                  ),
                  product_codes: Some(
                      [],
                  ),
                  public_dns_name: Some(
                      "ec2-3-21-106-28.us-east-2.compute.amazonaws.com",
                  ),
                  public_ip_address: Some(
                      "3.21.106.28",
                  ),
                  ramdisk_id: None,
                  root_device_name: Some(
                      "/dev/sda1",
                  ),
                  root_device_type: Some(
                      "ebs",
                  ),
                  security_groups: Some(
                      [
                          GroupIdentifier {
                              group_id: Some(
                                  "sg-06a99908748b8ea90",
                              ),
                              group_name: Some(
                                  "default",
                              ),
                          },
                      ],
                  ),
                  source_dest_check: Some(
                      true,
                  ),
                  spot_instance_request_id: None,
                  sriov_net_support: None,
                  state: Some(
                      InstanceState {
                          code: Some(
                              16,
                          ),
                          name: Some(
                              "running",
                          ),
                      },
                  ),
                  state_reason: None,
                  state_transition_reason: Some(
                      "",
                  ),
                  subnet_id: Some(
                      "subnet-0e6364007229332fe",
                  ),
                  tags: Some(
                      [
                          Tag {
                              key: Some(
                                  "e2e:resource",
                              ),
                              value: Some(
                                  "true",
                              ),
                          },
                      ],
                  ),
                  virtualization_type: Some(
                      "hvm",
                  ),
                  vpc_id: Some(
                      "vpc-0872734b71255b56c",
                  ),
              },
          ],
