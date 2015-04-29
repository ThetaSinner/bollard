#[cfg(test)]
pub fn get_containers_response() -> String {
    return "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n[{\"Command\":\"bash\",\"Created\":1428559770,\"Id\":\"71f4c0a989c590e2b19e9399c9ff5672767881d99e4cd409996de5995189ec33\",\"Image\":\"ghmlee/rust:nightly\",\"Names\":[\"/dreamy_lovelace\"],\"Ports\":[{\"IP\":\"0.0.0.0\",\"PrivatePort\":8083,\"PublicPort\":8083,\"Type\":\"tcp\"},{\"IP\":\"0.0.0.0\",\"PrivatePort\":8086,\"PublicPort\":8086,\"Type\":\"tcp\"},{\"PrivatePort\":8090,\"Type\":\"tcp\"},{\"PrivatePort\":8099,\"Type\":\"tcp\"},{\"PrivatePort\":8084,\"Type\":\"tcp\"}],\"SizeRootFs\":0,\"SizeRw\":10832473,\"Status\":\"Up 9 minutes\"}]".to_string();
}

#[cfg(test)]
pub fn get_stats_response() -> String {
    return "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"read\":\"2015-04-09T07:02:08.480022082Z\",\"network\":{\"rx_bytes\":5820720,\"rx_packets\":2742,\"rx_errors\":0,\"rx_dropped\":1,\"tx_bytes\":158527,\"tx_packets\":2124,\"tx_errors\":0,\"tx_dropped\":0},\"cpu_stats\":{\"cpu_usage\":{\"total_usage\":19194125000,\"percpu_usage\":[14110113138,3245604417,845722573,992684872],\"usage_in_kernelmode\":1110000000,\"usage_in_usermode\":18160000000},\"system_cpu_usage\":1014488290000000,\"throttling_data\":{\"periods\":0,\"throttled_periods\":0,\"throttled_time\":0}},\"memory_stats\":{\"usage\":208437248,\"max_usage\":318791680,\"stats\":{\"active_anon\":27213824,\"active_file\":129069056,\"cache\":178946048,\"hierarchical_memory_limit\":18446744073709551615,\"hierarchical_memsw_limit\":18446744073709551615,\"inactive_anon\":0,\"inactive_file\":49876992,\"mapped_file\":10809344,\"pgfault\":99588,\"pgmajfault\":819,\"pgpgin\":130731,\"pgpgout\":153466,\"rss\":29331456,\"rss_huge\":6291456,\"swap\":0,\"total_active_anon\":27213824,\"total_active_file\":129069056,\"total_cache\":178946048,\"total_inactive_anon\":0,\"total_inactive_file\":49876992,\"total_mapped_file\":10809344,\"total_pgfault\":99588,\"total_pgmajfault\":819,\"total_pgpgin\":130731,\"total_pgpgout\":153466,\"total_rss\":29331456,\"total_rss_huge\":6291456,\"total_swap\":0,\"total_unevictable\":0,\"total_writeback\":0,\"unevictable\":0,\"writeback\":0},\"failcnt\":0,\"limit\":16854257664},\"blkio_stats\":{\"io_service_bytes_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":150687744},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":150687744},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":150687744}],\"io_serviced_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":484},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":484},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":484}],\"io_queue_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":0}],\"io_service_time_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":2060941295},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":2060941295},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":2060941295}],\"io_wait_time_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":5476872825},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":5476872825},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":5476872825}],\"io_merged_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":79},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":79},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":79}],\"io_time_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"\",\"value\":1814}],\"sectors_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"\",\"value\":294312}]}}".to_string();
}

#[cfg(test)]
pub fn get_system_info_response() -> String {
    return "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"Containers\":6,\"Debug\":0,\"DockerRootDir\":\"/var/lib/docker\",\"Driver\":\"btrfs\",\"DriverStatus\":[[\"Build Version\",\"Btrfs v3.17.1\"],[\"Library Version\",\"101\"]],\"ExecutionDriver\":\"native-0.2\",\"ID\":\"WG63:3NIU:TSI2:FV7J:IL2O:YPXA:JR3F:XEKT:JZVR:JA6T:QMYE:B4SB\",\"IPv4Forwarding\":1,\"Images\":190,\"IndexServerAddress\":\"https://index.docker.io/v1/\",\"InitPath\":\"/usr/libexec/docker/dockerinit\",\"InitSha1\":\"30c93967bdc3634b6036e1a76fd547bbe171b264\",\"KernelVersion\":\"3.18.6\",\"Labels\":null,\"MemTotal\":16854257664,\"MemoryLimit\":1,\"NCPU\":4,\"NEventsListener\":0,\"NFd\":68,\"NGoroutines\":95,\"Name\":\"core\",\"OperatingSystem\":\"CoreOS 607.0.0\",\"RegistryConfig\":{\"IndexConfigs\":{\"docker.io\":{\"Mirrors\":null,\"Name\":\"docker.io\",\"Official\":true,\"Secure\":true}},\"InsecureRegistryCIDRs\":[\"127.0.0.0/8\"]},\"SwapLimit\":1}".to_string();
}

#[cfg(test)]
pub fn get_images_response() -> String {
    return "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n[{\"Created\":1428533761,\"Id\":\"533da4fa223bfbca0f56f65724bb7a4aae7a1acd6afa2309f370463eaf9c34a4\",\"ParentId\":\"84ac0b87e42afe881d36f03dea817f46893f9443f9fc10b64ec279737384df12\",\"RepoTags\":[\"ghmlee/rust:nightly\"],\"Size\":0,\"VirtualSize\":806688288},{\"Created\":1371157430,\"Id\":\"511136ea3c5a64f264b78b5433614aec563103b4d4702f3ba7d4d2698e22c158\",\"ParentId\":\"\",\"RepoTags\":[],\"Size\":0,\"VirtualSize\":0}]".to_string();
}

#[cfg(test)]
pub fn get_container_info_response() -> String {
    return "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"AppArmorProfile\":\"\",\"Args\":[],\"Config\":{\"AttachStderr\":false,\"AttachStdin\":false,\"AttachStdout\":false,\"Cmd\":[\"/run.sh\"],\"CpuShares\":0,\"Cpuset\":\"\",\"Domainname\":\"\",\"Entrypoint\":null,\"Env\":[\"HOME=/\",\"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\",\"INFLUXDB_VERSION=0.8.8\",\"PRE_CREATE_DB=**None**\",\"SSL_SUPPORT=**False**\",\"SSL_CERT=**None**\"],\"ExposedPorts\":{\"8083/tcp\":{},\"8084/tcp\":{},\"8086/tcp\":{},\"8090/tcp\":{},\"8099/tcp\":{}},\"Hostname\":\"a9de92dfbf97\",\"Image\":\"cosmosio/influxdb\",\"Labels\":{},\"MacAddress\":\"\",\"Memory\":0,\"MemorySwap\":0,\"NetworkDisabled\":false,\"OnBuild\":null,\"OpenStdin\":false,\"PortSpecs\":null,\"StdinOnce\":false,\"Tty\":false,\"User\":\"\",\"Volumes\":{\"/data\":{}},\"WorkingDir\":\"\"},\"Created\":\"2015-04-26T23:15:20.6051724Z\",\"Driver\":\"aufs\",\"ExecDriver\":\"native-0.2\",\"ExecIDs\":null,\"HostConfig\":{\"Binds\":null,\"CapAdd\":null,\"CapDrop\":null,\"CgroupParent\":\"\",\"ContainerIDFile\":\"\",\"CpuShares\":0,\"CpusetCpus\":\"\",\"Devices\":[],\"Dns\":null,\"DnsSearch\":null,\"ExtraHosts\":null,\"IpcMode\":\"\",\"Links\":null,\"LogConfig\":{\"Config\":null,\"Type\":\"json-file\"},\"LxcConf\":[],\"Memory\":0,\"MemorySwap\":0,\"NetworkMode\":\"bridge\",\"PidMode\":\"\",\"PortBindings\":{\"8083/tcp\":[{\"HostIp\":\"\",\"HostPort\":\"8083\"}],\"8086/tcp\":[{\"HostIp\":\"\",\"HostPort\":\"8086\"}]},\"Privileged\":false,\"PublishAllPorts\":false,\"ReadonlyRootfs\":false,\"RestartPolicy\":{\"MaximumRetryCount\":0,\"Name\":\"no\"},\"SecurityOpt\":null,\"Ulimits\":null,\"VolumesFrom\":null},\"HostnamePath\":\"/mnt/sda1/var/lib/docker/containers/a9de92dfbf9739aa945efbeafb8112d9dd8b986a724185afe1cdca3ab2ff4a3c/hostname\",\"HostsPath\":\"/mnt/sda1/var/lib/docker/containers/a9de92dfbf9739aa945efbeafb8112d9dd8b986a724185afe1cdca3ab2ff4a3c/hosts\",\"Id\":\"a9de92dfbf9739aa945efbeafb8112d9dd8b986a724185afe1cdca3ab2ff4a3c\",\"Image\":\"bd6edeff2eb78594a5a48d498efc7ef04cafb126c37e5ae7533c9f243985742a\",\"LogPath\":\"/mnt/sda1/var/lib/docker/containers/a9de92dfbf9739aa945efbeafb8112d9dd8b986a724185afe1cdca3ab2ff4a3c/a9de92dfbf9739aa945efbeafb8112d9dd8b986a724185afe1cdca3ab2ff4a3c-json.log\",\"MountLabel\":\"\",\"Name\":\"/influxdb\",\"NetworkSettings\":{\"Bridge\":\"docker0\",\"Gateway\":\"172.17.42.1\",\"GlobalIPv6Address\":\"\",\"GlobalIPv6PrefixLen\":0,\"IPAddress\":\"172.17.0.2\",\"IPPrefixLen\":16,\"IPv6Gateway\":\"\",\"LinkLocalIPv6Address\":\"fe80::42:acff:fe11:2\",\"LinkLocalIPv6PrefixLen\":64,\"MacAddress\":\"02:42:ac:11:00:02\",\"PortMapping\":null,\"Ports\":{\"8083/tcp\":[{\"HostIp\":\"0.0.0.0\",\"HostPort\":\"8083\"}],\"8084/tcp\":null,\"8086/tcp\":[{\"HostIp\":\"0.0.0.0\",\"HostPort\":\"8086\"}],\"8090/tcp\":null,\"8099/tcp\":null}},\"Path\":\"/run.sh\",\"ProcessLabel\":\"\",\"ResolvConfPath\":\"/mnt/sda1/var/lib/docker/containers/a9de92dfbf9739aa945efbeafb8112d9dd8b986a724185afe1cdca3ab2ff4a3c/resolv.conf\",\"RestartCount\":0,\"State\":{\"Dead\":false,\"Error\":\"\",\"ExitCode\":0,\"FinishedAt\":\"0001-01-01T00:00:00Z\",\"OOMKilled\":false,\"Paused\":false,\"Pid\":891,\"Restarting\":false,\"Running\":true,\"StartedAt\":\"2015-04-26T23:15:21.034009864Z\"},\"Volumes\":{\"/data\":\"/mnt/sda1/var/lib/docker/vfs/dir/e5519f5fe7434608f575d84a532cc7ee0a16b792d78679fe34b7d04269469694\"},\"VolumesRW\":{\"/data\":true}}".to_string();
}
