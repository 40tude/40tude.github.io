---
layout: default
title: "Format and use USB key"
parent: "Linux"
#math: mathjax
date: 2023-11-30 16:16:05
last_modified_date: 2023-11-30 16:16:05
---

# Format and use USB key

Tests have been done with Linux Mint 21.2 but the process does not depend on the distribution.

* Plug the key.
* Once detected, Nemo or any other "file explorer" opens.
* "CTRL+ALT+T" to open a terminal.

```
lsblk                                       # No need to be root
                                            # You see sdb1 or sdb1 and sdb2 (my case)
                                            # They are mounted on /media/philippe/xxxxx and /media/philippe/yyyyyyy
umount /dev/sdb1                            #
umount /dev/sdb2
lsblk                                       # No mounting point anymore
```

{: .note }
I suppose the USB stick is under /dev/sdb

## If the USB stick must be bootable

* Create a partition table, skip this step otherwise

```
sudo fdisk /dev/sdb
g                                           # Create GPT - Globally Unique Identifier Partition Table
```
## I want to create 2 partitions

* If fdisk is not already in use, run it : `sudo fdisk /dev/sdb`
* The letters below (p, d...) are the key to use once under fdisk

```
p                                           # print partitions. Double check you're on the USB and NOT you main drive
d                                           # delete partition
n                                           # new partition (p,1,2048,+200M)
n                                           # new partition (keep default values: p...)
p                                           # double check
w                                           # write and quit fdisk
```

## Create a filesystem

```
sudo fdisk -l                               # Shows the 2 partitions sdb1 et sdb2
sudo mkfs.ext4 /dev/sdb1
sudo mkfs.ext4 /dev/sdb2
sudo fsck /dev/sdb1                         # checking
sudo fsck /dev/sdb2
```

## Only if you're paranoid, skip otherwise

```
sudo dd if=/dev/zero of=/dev/sdb1 bs=1M     # write 0
sudo shred -n 3 -z /dev/sdb1                # 3 times, zeroes the partition

```

## If one day you want to remove the GPT

```
sudo gdisk /dev/sdb
?                                          # To see the menu
i                                          # To check partitions details information
                                           # Use x then z options to zap the GPT data structure
```

At this stage you can unplug and plug the key or...

## Mount the USB key

```
sudo mkdir /media/usb1                              # The dirs belong to root
sudo mkdir /media/usb2                              # Remember the Alamo and the fact that I have 2 partition on the stick
sudo chown -R philippe:philippe /media/usb1         # change ownership
sudo chown -R philippe:philippe /media/usb2
sudo mount /dev/sdb1 /media/usb1
sudo mount /dev/sdb2 /media/usb2
```

* Use the key as philippe

## Unmount the USB key

```
sudo umount /media/usb1
sudo umount /media/usb2
sudo rmdir /media/usb1
sudo rmdir /media/usb2
```

* Unplug the USB key

